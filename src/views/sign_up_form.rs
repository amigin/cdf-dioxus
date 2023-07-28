use dioxus::prelude::*;
use my_telemetry::MyTelemetryContext;

use crate::{
    lang::LANG, states::GlobalState, trader_credentials_grpc::VerifyTraderPasswordRequest,
};

pub fn sign_up_form(cx: Scope) -> Element {
    let user_name = use_state(cx, || "".to_string());
    let password = use_state(cx, || "".to_string());

    let global_state = use_shared_state::<GlobalState>(cx).unwrap();

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
                                    class: "btn btn-light",
                                    onclick: move |_| {
                                        let global_state = global_state.to_owned();
                                        global_state.write().set_root();
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

                                div {
                                    label { class: "form-label", "Password again" }
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
                                    do_request(cx, user_name.get(), password.get(), global_state);
                                },
                                "SIGN UP"
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
) {
    let username = username.to_string();
    let password = password.to_string();

    let global_state = global_state.to_owned();

    cx.spawn(async move {
        let result = tokio::spawn(async move {
            let app = crate::APP_CTX.get().await;

            let result = app
                .trader_credentials_grpc_client
                .verify_password(
                    VerifyTraderPasswordRequest {
                        email: username,
                        password: password,
                        brand: app.get_brand().await,
                    },
                    &MyTelemetryContext::new(),
                )
                .await
                .unwrap();

            result.trader_id

            //http::login(&username, &password).await
        })
        .await
        .unwrap();

        if let Some(trader_id) = result {
            global_state.write().set_loading(trader_id.into());
        } else {
            println!("Invalid Username or password");
        }
    });
}
