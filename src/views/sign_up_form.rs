use super::*;
use dioxus::prelude::*;
use dioxus_toast::ToastManager;
use fermi::{use_atom_ref, UseAtomRef};

use crate::{
    grpc_client::TraderCredentialsGrpcClient, lang::LANG, session_token::SessionToken,
    states::GlobalState, APP_CTX,
};

pub fn sign_up_form(cx: Scope) -> Element {
    let email = use_state(cx, || "".to_string());
    let password = use_state(cx, || "".to_string());
    let password_again = use_state(cx, || "".to_string());

    let global_state = use_shared_state::<GlobalState>(cx).unwrap();

    let sign_up_form = &LANG.sign_up_form;
    let toast = use_atom_ref(cx, &crate::TOAST_MANAGER);

    let password_again_element = use_state(cx, || None);

    let (passwords_do_not_match_phrase, class_name, mut button_disable) =
        if password.get() != password_again.get() {
            let result =
                rsx!( div { style: "color:red", "{LANG.toast_errors.passwords_do_not_match}" } );
            (result, "validation-error", true)
        } else {
            (rsx!(div {}), "", false)
        };

    if !crate::validators::is_email(email.get()) {
        button_disable = true;
    }

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
                                    class: "btn btn-light",
                                    onclick: move |_| {
                                        let global_state = global_state.to_owned();
                                        global_state.write().set_login();
                                    },
                                    "{LANG.login}"
                                }
                                button {
                                    r#type: "button",
                                    style: "width:140px",
                                    class: "btn btn-success",
                                    "{LANG.sign_up}"
                                }
                            }
                            div { style: "text-align: left;margin-top: 30px;",

                                label { class: "form-label", "{sign_up_form.email}" }
                                input {
                                    id: "email",
                                    r#type: "email",
                                    class: "form-control",
                                    placeholder: "{sign_up_form.email}",
                                    oninput: move |e| {
                                        email.set(e.value.trim().to_lowercase());
                                    }
                                }

                                div { style: "margin-top:10px",
                                    label { class: "form-label", "{sign_up_form.password}" }
                                    input {
                                        r#type: "{sign_up_form.password}",
                                        class: "form-control",
                                        placeholder: "{sign_up_form.password}",
                                        oninput: move |e| {
                                            password.set(e.value.clone());
                                        }
                                    }
                                }

                                div { style: "margin-top:10px",
                                    label { class: "form-label", "{sign_up_form.password_again}" }
                                    passwords_do_not_match_phrase,
                                    input {
                                        r#type: "{sign_up_form.password}",
                                        class: "form-control {class_name}",
                                        placeholder: "{sign_up_form.password_again}",

                                        onmounted: move |e| {
                                            password_again_element.set(Some(e.inner().clone()));
                                        },

                                        oninput: move |e| {
                                            password_again.set(e.value.clone());
                                        }
                                    }
                                }
                            }

                            button {

                                r#type: "button",

                                style: "width:100%; margin-top: 30px;",

                                class: "btn btn-success",

                                disabled: button_disable,

                                onclick: move |_| {
                                    let password = password.get();
                                    do_request(cx, email.get(), password, global_state, toast);
                                },
                                "{LANG.sign_up}"
                            }

                            forgot_password_link {}
                        }
                    }
                }
            }
        }
        script { "set_focus('email');" }
    }
}

fn do_request(
    cx: &Scoped,
    email: &str,
    password: &str,
    global_state: &UseSharedState<GlobalState>,
    toast: &UseAtomRef<ToastManager>,
) {
    let email = email.to_string();
    let password = password.to_string();

    let global_state = global_state.to_owned();

    let toast = toast.to_owned();

    cx.spawn(async move {
        let result =
            TraderCredentialsGrpcClient::register_new_client(email.to_string(), password).await;

        match result {
            Ok(trader_id) => {
                let aes_key = APP_CTX.get_aes_key().await;

                let session_token = SessionToken::new(trader_id.as_str().to_string());
                let session_token = session_token.to_string(&aes_key);

                global_state
                    .write()
                    .set_loading(trader_id, email, session_token);
            }
            Err(err) => {
                err.throw_toast(&LANG.toast_errors.registration_fail, &toast);
            }
        }
    });
}
