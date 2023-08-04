use dioxus::prelude::*;

use crate::{lang::LANG, states::GlobalState};

pub fn forgot_password_link(cx: Scope) -> Element {
    let global_state = use_shared_state::<GlobalState>(cx).unwrap();

    render! {
        div { style: "text-align: center; margin-top: 30px;",
            a {
                style: "cursor: pointer; ",
                onclick: move |_| {
                    let global_state = global_state.to_owned();
                    global_state.write().set_reset_password();
                },
                "{LANG.forgot_password}?"
            }
        }
    }
}
