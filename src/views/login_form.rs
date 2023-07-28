use dioxus::prelude::*;
use dioxus_toast::{ToastInfo, ToastManager};

use fermi::{use_atom_ref, UseAtomRef};

use crate::{grpc_client::TraderCredentialsGrpcClient, lang::LANG, states::GlobalState};

pub fn login_form(cx: Scope) -> Element {
    let user_name = use_state(cx, || "".to_string());
    let password = use_state(cx, || "".to_string());

    let global_state = use_shared_state::<GlobalState>(cx).unwrap();

    let toast = use_atom_ref(&cx, crate::TOAST_MANAGER);

    render! {
        table { class: "table-layout",
            tr { style: "width:100%",
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
                                    r#type: "email",
                                    class: "form-control",
                                    placeholder: "email",
                                    oninput: move |e| {
                                        user_name.set(e.value.clone());
                                    }
                                }

                                div {
                                    label { class: "form-label", "Password" }
                                    input {
                                        r#type: "{LANG.password}",
                                        class: "form-control",
                                        placeholder: "{LANG.password}",
                                        oninput: move |e| {
                                            password.set(e.value.clone());
                                        }
                                    }
                                }
                            }

                            button {

                                r#type: "button",

                                style: "width:100%; margin-top: 30px;",

                                class: "btn btn-success",

                                onclick: move |_| {
                                    do_request(cx, user_name.get(), password.get(), global_state, toast);
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
    username: &str,
    password: &str,
    global_state: &UseSharedState<GlobalState>,
    toast: &UseAtomRef<ToastManager>,
) {
    let username = username.to_string();
    let password = password.to_string();

    let global_state = global_state.to_owned();

    let toast = toast.to_owned();

    cx.spawn(async move {
        let result = TraderCredentialsGrpcClient::check_password(username, password).await;

        match result {
            Ok(trader_id) => {
                println!("Trader id: {}", trader_id);
                global_state.write().set_loading(trader_id);
            }
            Err(err) => {
                toast
                    .write()
                    .popup(ToastInfo::error(err.as_str(), &LANG.login_failed));
            }
        }
    });
}
