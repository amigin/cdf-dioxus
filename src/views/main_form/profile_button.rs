use dioxus::prelude::*;

use crate::{states::MainFormState, views::icons::*};

pub fn profile_button(cx: Scope) -> Element {
    let main_form_state = use_shared_state::<MainFormState>(cx).unwrap();

    let mut result: Vec<LazyNodes<'_, '_>> = Vec::with_capacity(2);
    let button = rsx! {
        div { style: " margin-right: 6px;",
            button {
                class: "btn btn-outline-dark btn-sm dropdown-toggle",

                onclick: move |_| {
                    main_form_state.write().click_profile_menu();
                },

                span {
                    person_icon {}
                    "Profile"
                }
            }
        }
    };

    result.push(button);

    if main_form_state.read().profile_menu_is_shown() {
        let menu = rsx! {
            div { id: "profile-menu", class: "floating-menu",
                div { style: "display: flex",
                    div { style: "display: flex; flex-grow:1", profile_menu_icon {} }
                    div {
                        onclick: move |_| {
                            main_form_state.write().hide_dialog();
                        },
                        style: "display: flex; cursor: pointer;",
                        close_icon {}
                    }
                }
                hr {}
                a { class: "dropdown-item", href: "#", "Account settings" }
                a { class: "dropdown-item", href: "#", "Deposit" }
                a { class: "dropdown-item", href: "#", "Withdraw" }
                a { class: "dropdown-item", href: "#", "Balance history" }
                a { class: "dropdown-item", href: "#", "Logout" }
            }
        };

        result.push(menu);
    }

    render! {result.into_iter()}
}
