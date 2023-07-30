use dioxus::prelude::*;

use crate::views::settings_form::withdraw_frame::*;

enum SelectedTab {
    Request,
    History,
}

pub fn withdraw_frame(cx: Scope) -> Element {
    let state = use_state(cx, || SelectedTab::Request);

    let (request_class, history_class, frame) = match state.get() {
        SelectedTab::Request => (
            "withdraw-tab-active",
            "withdraw-tab",
            rsx! {bank_transfer_frame{}},
        ),
        SelectedTab::History => (
            "withdraw-tab",
            "withdraw-tab-active",
            rsx! {withdraw_history_frame{}},
        ),
    };

    render! {
        table { style: "width:708px; text-align: center;",
            tr {
                td { class: request_class,
                    div {
                        class: "content",
                        onclick: move |_| {
                            state.set(SelectedTab::Request);
                        },
                        "Request"
                    }
                }
                td { class: history_class,
                    div {
                        class: "content",
                        onclick: move |_| {
                            state.set(SelectedTab::History);
                        },
                        "History"
                    }
                }
            }
        }

        frame
    }
}
