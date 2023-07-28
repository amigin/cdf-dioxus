use std::net::SocketAddr;
use std::sync::Arc;

use crate::app::AppContext;
use crate::{states::*, views::*};
use dioxus::prelude::*;
use dioxus_liveview::LiveViewPool;
use dioxus_toast::ToastManager;
use fermi::{use_atom_ref, use_init_atom_root, AtomRef};

use salvo::affix;
use salvo::prelude::*;

mod actions;
mod app;
mod grpc_client;
mod http_server;
mod lang;
mod settings_reader;
mod states;
mod static_resources;
mod types;
mod views;

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

    let addr: SocketAddr = ([127, 0, 0, 1], 9001).into();
    let acceptor = TcpListener::bind(addr);
    let view = LiveViewPool::new();

    let router = Router::new()
        .hoop(affix::inject(Arc::new(view)))
        .get(http_server::index)
        .push(Router::with_path("ws").get(http_server::connect));

    println!("Listening on http://{}", addr);

    Server::new(acceptor).serve(router).await;
}
static TOAST_MANAGER: AtomRef<ToastManager> = |_| ToastManager::default();

fn app(cx: Scope) -> Element {
    use_shared_state_provider(cx, || GlobalState::Loading {
        trader_id: "2808ee4d-4231-4697-b892-eb95741c811c".into(),
    });

    //use_shared_state_provider(cx, || GlobalState::NonAuthenticated);

    use_shared_state_provider(cx, || AccountsState::new());
    use_shared_state_provider(cx, || FavInstrumentsState::new());

    use_shared_state_provider(cx, || InstrumentsState::new());

    use_init_atom_root(&cx);

    std::panic::set_hook(Box::new(|info| {
        println!("Panic: {}", info);
    }));

    let toast = use_atom_ref(&cx, TOAST_MANAGER);

    let state: &UseSharedState<GlobalState> = use_shared_state::<GlobalState>(cx).unwrap();
    /*
    let mut toast_sender = ToastSender::new();

    let mut receiver = toast_sender.get_receiver();

    use_shared_state_provider(cx, || Arc::new(toast_sender));


       cx.spawn(async move {
           while let Some(value) = receiver.recv().await {
               let _id = toast
                   .write()
                   .popup(ToastInfo::error(&value.body, &value.header));
           }
       });
    */

    match state.read().as_ref() {
        GlobalState::NonAuthenticated => render! {
            dioxus_toast::ToastFrame { manager: toast }
            login_form {}
        },
        GlobalState::SignUp => render!(
            dioxus_toast::ToastFrame { manager: toast }
            sign_up_form {}
        ),
        GlobalState::Loading { .. } => render! {
            dioxus_toast::ToastFrame { manager: toast }
            loading_form {}
        },
        GlobalState::Authenticated { .. } => render! {
            dioxus_toast::ToastFrame { manager: toast }
            main_form {}
        },
    }
}
