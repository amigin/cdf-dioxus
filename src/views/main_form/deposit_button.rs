use dioxus::prelude::*;

use crate::states::MainFormState;

pub fn deposit_button(cx: Scope) -> Element {
    let main_form_state = use_shared_state::<MainFormState>(cx).unwrap();

    let mut result = Vec::with_capacity(2);
    result.push(rsx! {
        div { style: "margin-right: 6px;",
            button {
                class: "btn btn-success btn-sm",
                onclick: move |_| {
                    main_form_state.write().show_deposit_panel();
                },

                "Deposit"
            }
        }
    });

    if main_form_state.read().deposit_is_shown() {
        let content = rsx! {
            div {
                id: "dialogBackground",
                onclick: move |_| {
                    main_form_state.write().hide_dialog();
                },

                div { id: "depositPanel", onclick: |e| {
                        e.stop_propagation();
                    },
                    div { class: "dialog-header",
                        div { class: "dialog-header-title", "Deposit" }
                        div { style: "padding: 10px;",
                            button {
                                r#type: "button",
                                onclick: move |_| {
                                    main_form_state.write().hide_dialog();
                                },
                                class: "btn-close"
                            }
                        }
                    }
                    div { class: "deposit-dialog-compliance-content",
                        div { style: "margin: auto;",
                            "You have to pass the compliance to get deposit methods."
                        }
                    }
                    div { class: "deposit-secure-panel",
                        img { style: "height:32px;", src: "/img/ssl-img.svg" }
                        img {
                            style: "padding-left:20px; height:32px",
                            src: "/img/mastercard-check.png"
                        }

                        img {
                            style: "padding-left:20px; height:32px",
                            src: "/img/visa-secure.svg"
                        }
                    }
                }
            }
        };

        result.push(content);
    }

    render! {result.into_iter()}
}
