use std::sync::Arc;

use crate::app::AppContext;
use crate::{states::*, views::*};
use dioxus::prelude::*;
use dioxus_liveview::LiveViewPool;
use dioxus_toast::ToastManager;
use fermi::{use_atom_ref, use_init_atom_root, AtomRef};

use salvo::affix;
use salvo::prelude::*;
use salvo::serve_static::StaticDir;

mod actions;
mod app;
mod grpc_client;
mod http_server;
mod lang;
mod session_token;
mod settings_reader;
mod states;
mod static_resources;
mod types;
mod validators;
mod views;

pub const MOCK_CURRENCY_USD: &'static str = "$";

lazy_static::lazy_static! {
    pub static ref APP_CTX: Arc<AppContext> = {
        Arc::new(AppContext::new())
    };
}

pub mod trader_credentials_grpc {
    tonic::include_proto!("trader_credentials");
}

pub mod accounts_manager_grpc {
    tonic::include_proto!("accounts_manager");
}

pub mod favorite_instruments_flows_grpc {
    tonic::include_proto!("favorite_instruments_flows");
}

pub mod keyvalue_grpc {
    tonic::include_proto!("keyvalue");
}

#[tokio::main]
async fn main() {
    /*
    let head = static_resources::get_header_content();
    dioxus_desktop::launch_cfg(app, dioxus_desktop::Config::new().with_custom_head(head));
    */

    let settings = crate::settings_reader::SettingsReader::new(".cfd-web-terminal").await;

    APP_CTX.apply_settings(Arc::new(settings)).await;

    let acceptor = TcpListener::new("0.0.0.0:9001").bind().await;
    let view = LiveViewPool::new();

    let router = Router::new()
        .hoop(affix::inject(Arc::new(view)))
        .get(http_server::index)
        .push(Router::with_path("ws").get(http_server::connect))
        .push(Router::with_path("img/<**path>").get(StaticDir::new("./files/img")))
        .push(Router::with_path("avatar/<**path>").get(http_server::get_avatar));

    Server::new(acceptor).serve(router).await;
}
static TOAST_MANAGER: AtomRef<ToastManager> = AtomRef(|_| ToastManager::default());

fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || GlobalState::NonAuthenticated {
        reset_session: false,
    });

    //use_shared_state_provider(cx, || GlobalState::NonAuthenticated);

    use_shared_state_provider(cx, || AccountsState::new());
    use_shared_state_provider(cx, || FavInstrumentsState::new());

    use_shared_state_provider(cx, || InstrumentsState::new());

    use_init_atom_root(&cx);

    std::panic::set_hook(Box::new(|info| {
        println!("Panic: {}", info);
    }));

    let toast = use_atom_ref(cx, &TOAST_MANAGER);

    let state: &UseSharedState<GlobalState> = use_shared_state::<GlobalState>(cx).unwrap();

    match state.read().as_ref() {
        GlobalState::NonAuthenticated { reset_session } => render! {
            dioxus_toast::ToastFrame { manager: toast }
            login_form { reset_session: *reset_session }
        },
        GlobalState::SignUp => render!(
            dioxus_toast::ToastFrame { manager: toast }
            sign_up_form {}
        ),
        GlobalState::Loading {
            trader_id,
            email,
            session_token,
        } => {
            render! {
                dioxus_toast::ToastFrame { manager: toast }

                loading_form {
                    trader_id: trader_id.clone(),
                    email: email.to_string(),
                    session_token: session_token.clone()
                }
            }
        }
        GlobalState::Authenticated { .. } => render! {
            dioxus_toast::ToastFrame { manager: toast }
            main_form {}
        },
        GlobalState::ResetPassword => render! {
            dioxus_toast::ToastFrame { manager: toast }
            reset_password_form {}
        },
    }
}
