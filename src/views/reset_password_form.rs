use dioxus::prelude::*;

use crate::{lang::LANG, states::GlobalState};

enum WindowMode {
    EnteringEmail,
    EnteringCode,
}

pub fn reset_password_form(cx: Scope) -> Element {
    let lang = &LANG.reset_password_form;
    let global_state = use_shared_state::<GlobalState>(cx).unwrap();

    let mode = use_state(cx, || WindowMode::EnteringEmail);

    let component = match mode.get() {
        WindowMode::EnteringEmail => rsx! {
            type_email {
                on_enter: |_| {
                    mode.set(WindowMode::EnteringCode);
                }
            }
        },
        WindowMode::EnteringCode => rsx! { type_code {} },
    };

    render! {
        table { style: "width:100%;height:100vh;",
            tr {
                td {
                    div { id: "reset-password-form",

                        h1 { "{lang.reset_password_title}" }

                        component,

                        div { style: "margin-top:10px; cursor: pointer",
                            a { onclick: move |_| {
                                    global_state.write().set_login();
                                },
                                "{lang.back_to_login}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props)]
pub struct TypeEmailProps<'s> {
    pub on_enter: EventHandler<'s, String>,
}

fn type_email<'s>(cx: Scope<'s, TypeEmailProps<'s>>) -> Element<'s> {
    let email = use_state(cx, || "".to_string());
    let lang = &LANG.reset_password_form;

    let mut disable_button = false;

    if !crate::validators::is_email(email.get()) {
        disable_button = true;
    }
    render! {
        div { "{lang.to_begin_changing_password_under_title}" }
        div { style: "margin-top: 20px;",
            input {

                r#type: "email",

                class: "form-control",
                oninput: move |e| {
                    email.set(e.value.trim().to_lowercase());
                },
                placeholder: "{lang.email_input}"
            }
            button {
                style: "margin-top:10px",
                class: "btn btn-success",
                disabled: disable_button,
                onclick: move |_| {
                    cx.props.on_enter.call(email.get().to_string());
                },
                "{lang.send_recovery_code}"
            }
        }
    }
}

fn type_code(cx: Scope) -> Element {
    render! { h1 { "Type code" } }
}
