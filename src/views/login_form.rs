use dioxus::prelude::*;
use dioxus_toast::ToastManager;

use fermi::{use_atom_ref, UseAtomRef};
use rust_extensions::date_time::DateTimeAsMicroseconds;

use super::*;

use crate::{
    grpc_client::TraderCredentialsGrpcClient, lang::LANG, session_token::SessionToken,
    states::GlobalState, APP_CTX,
};

pub enum LoginFormState {
    LoadingDataFromBrowser,
    RenderLoginForm(String),
}
#[derive(Props, PartialEq)]
pub struct LoginFormProps {
    pub reset_session: bool,
}

pub fn login_form<'s>(cx: Scope<'s, LoginFormProps>) -> Element<'s> {
    let eval = use_eval(cx);
    let eval_owned = eval.to_owned();

    let we_got_result = use_state(cx, || false);

    let we_got_result_owned = we_got_result.to_owned();

    let global_state = use_shared_state::<GlobalState>(cx).unwrap();

    let global_state_owned = global_state.to_owned();

    let login_form_state = use_state(cx, || LoginFormState::LoadingDataFromBrowser);

    let login_form_state_owned = login_form_state.to_owned();

    match login_form_state.get() {
        LoginFormState::LoadingDataFromBrowser => {
            cx.spawn(async move {
                if *we_got_result_owned.get() {
                    return;
                }

                let eval = eval_owned(
                    r#"
                let email = localStorage.getItem('email');
                let session = localStorage.getItem('session_id');
                if (!email){
                    email = '-';
                }
                if (!session){
                    session = '-';
                }
        
                return email + ' ' + session;
                "#,
                )
                .unwrap();

                let result = eval.await;

                we_got_result_owned.set(true);

                match result {
                    Ok(result) => {
                        if let Some(value) = result.as_str() {
                            let parts: Vec<&'_ str> = value.split(' ').collect();

                            let email = *parts.get(0).unwrap();

                            if parts.len() > 1 {
                                let session_id = *parts.get(1).unwrap();

                                if session_id.len() > 2 {
                                    let token = SessionToken::from_string(
                                        session_id,
                                        &APP_CTX.get_aes_key().await,
                                    );

                                    if let Ok(token) = token {
                                        if !token.is_expired(DateTimeAsMicroseconds::now()) {
                                            global_state_owned.write().set_loading(
                                                token.trader_id.into(),
                                                "".to_string(),
                                                "".to_string(),
                                            );
                                        }
                                    }
                                }
                            }

                            if email == "-" {
                                login_form_state_owned
                                    .set(LoginFormState::RenderLoginForm("".to_string()));
                            } else {
                                login_form_state_owned
                                    .set(LoginFormState::RenderLoginForm(email.to_string()));
                            }
                            return;
                        }
                    }
                    Err(err) => {
                        println!("Error: {:?}", err);
                    }
                }

                login_form_state_owned.set(LoginFormState::RenderLoginForm("".to_string()));
            });

            let script = if cx.props.reset_session {
                rsx! { script { "localStorage.removeItem('session_id');" } }
            } else {
                rsx!(div {})
            };

            render! {
                h1 { "Initializing..." }
                script
            }
        }
        LoginFormState::RenderLoginForm(email) => {
            render! { render_login_form { email: email.clone() } }
        }
    }
}

#[derive(Props, PartialEq)]
pub struct RenderLoginFormProps {
    pub email: String,
}

fn render_login_form<'s>(cx: Scope<'s, RenderLoginFormProps>) -> Element<'s> {
    let email_value = use_state(cx, || cx.props.email.to_string());
    let password_value = use_state(cx, || "".to_string());

    let global_state = use_shared_state::<GlobalState>(cx).unwrap();

    let request_is_going_on = use_state(cx, || false);

    let toast = use_atom_ref(cx, &crate::TOAST_MANAGER);

    let mut button_is_disabled = false;

    if !crate::validators::is_email(email_value.get()) {
        button_is_disabled = true;
    }

    if password_value.get().len() == 0 {
        button_is_disabled = true;
    }

    if *request_is_going_on.get() {
        button_is_disabled = true;
    }

    let set_focus_js = if cx.props.email.len() < 2 {
        "set_focus('email');"
    } else {
        "set_focus('password');"
    };

    render! {
        div { class: "login-logo", img { src: "/img/Logo-green.png" } }
        table { class: "table-layout",
            tr { style: "width:100%;height: 100vh;",
                td {

                    div { class: "card", style: "width: 400px; margin: auto; ",
                        div { class: "card-body", style: "box-shadow: 0 0 5px lightgray",

                            div { class: "btn-group", role: "group",
                                button {
                                    r#type: "button",
                                    style: "width:140px",
                                    class: "btn btn-success",
                                    "{LANG.login}"
                                }
                                button {
                                    r#type: "button",
                                    style: "width:140px",
                                    class: "btn btn-light",
                                    onclick: move |_| {
                                        let global_state = global_state.to_owned();
                                        global_state.write().set_sign_up();
                                    },
                                    "{LANG.sign_up}"
                                }
                            }
                            div { style: "text-align: left;margin-top: 30px;",

                                label { class: "form-label", "Username" }
                                input {
                                    id: "email",
                                    r#type: "email",
                                    class: "form-control",
                                    placeholder: "email",
                                    value: "{email_value.get()}",

                                    oninput: move |e| {
                                        email_value.set(e.value.trim().to_lowercase());
                                    }
                                }

                                div {
                                    label { class: "form-label", "Password" }
                                    input {
                                        id: "password",
                                        r#type: "{LANG.password}",
                                        class: "form-control",
                                        placeholder: "{LANG.password}",
                                        oninput: move |e| {
                                            password_value.set(e.value.to_string());
                                        }
                                    }
                                }
                            }

                            button {

                                r#type: "button",

                                style: "width:100%; margin-top: 30px;",

                                class: "btn btn-success",
                                disabled: button_is_disabled,

                                onclick: move |_| {
                                    request_is_going_on.set(true);
                                    do_request(
                                        cx,
                                        email_value.get(),
                                        password_value.get(),
                                        global_state,
                                        toast,
                                        request_is_going_on,
                                    );
                                },
                                "{LANG.login}"
                            }

                            forgot_password_link {}
                            script { set_focus_js }
                        }
                    }
                }
            }
        }
    }
}

fn do_request<'s>(
    cx: &'s Scoped<'s, RenderLoginFormProps>,
    email: &str,
    password: &str,
    global_state: &UseSharedState<GlobalState>,
    toast: &UseAtomRef<ToastManager>,
    request_is_going_on: &UseState<bool>,
) {
    let email = email.to_string();
    let password = password.to_string();

    let global_state = global_state.to_owned();

    let toast = toast.to_owned();

    let request_is_going_on = request_is_going_on.to_owned();

    cx.spawn(async move {
        let result = TraderCredentialsGrpcClient::check_password(email.clone(), password).await;

        match result {
            Ok(trader_id) => {
                let aes_key = APP_CTX.get_aes_key().await;

                let session_token = SessionToken::new(trader_id.as_str().to_string());
                let session_token = session_token.to_string(&aes_key);

                global_state
                    .write()
                    .set_loading(trader_id, email.to_string(), session_token);
            }
            Err(err) => {
                request_is_going_on.set(false);
                err.throw_toast(&LANG.toast_errors.authentication_fail, &toast);
            }
        }
    });
}
