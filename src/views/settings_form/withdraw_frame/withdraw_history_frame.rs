use dioxus::prelude::*;

pub fn withdraw_history_frame(cx: Scope) -> Element {
    render! {
        div { style: "margin-top:20px; text-align:center;",
            label { "You haven't made any withdrawal requests yet" }
        }
    }
}
