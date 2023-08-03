use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_toast::ToastManager;

use fermi::{use_atom_ref, UseAtomRef};

use crate::{grpc_client::TraderCredentialsGrpcClient, lang::LANG, states::GlobalState};

pub fn login_form(cx: Scope) -> Element {
    let email_element: &UseState<Option<Rc<MountedData>>> = use_state(cx, || None);

    let email_element_owned = email_element.to_owned();

    let password_element: &UseState<Option<Rc<MountedData>>> = use_state(cx, || None);

    let password_element_owned = password_element.to_owned();

    let eval = use_eval(cx);
    let eval_owned = eval.to_owned();
    let email_value = use_state(cx, || "".to_string());
    let email_value_owned = email_value.to_owned();

    let we_got_result = use_state(cx, || false);

    let we_got_result_owned = we_got_result.to_owned();

    cx.spawn(async move {
        if *we_got_result_owned.get() {
            return;
        }

        let eval = eval_owned(r#"return localStorage.getItem('email');"#).unwrap();

        let result = eval.await;

        we_got_result_owned.set(true);

        match result {
            Ok(result) => {
                if let Some(value) = result.as_str() {
                    email_value_owned.set(value.to_string());
                    if let Some(el) = password_element_owned.get() {
                        el.set_focus(true);
                    }
                    return;
                }
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }

        if let Some(el) = email_element_owned.get() {
            el.set_focus(true);
        }
    });

    let password_value = use_state(cx, || "".to_string());

    let request_is_going_on = use_state(cx, || false);

    let global_state = use_shared_state::<GlobalState>(cx).unwrap();

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

                                    onmounted: move |e| {
                                        email_element.set(Some(e.inner().clone()));
                                    },

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
                                        onmounted: move |e| {
                                            password_element.set(Some(e.inner().clone()));
                                        },
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

                            div { style: "text-align: center; margin-top: 30px;",
                                a { href: "#", "{LANG.forgot_password}?" }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn do_request(
    cx: &Scoped,
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
                global_state
                    .write()
                    .set_loading(trader_id, email.to_string());
            }
            Err(err) => {
                request_is_going_on.set(false);
                err.throw_toast(&LANG.toast_errors.authentication_fail, &toast);
            }
        }
    });
}
