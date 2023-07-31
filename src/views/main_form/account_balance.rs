use dioxus::prelude::*;

use crate::{lang::LANG, states::*, widgets::*};

pub fn account_balance(cx: Scope) -> Element {
    let accounts_state = use_shared_state::<AccountsState>(cx).unwrap();

    let (currency, balance, is_live) = {
        let read_access = accounts_state.read();

        let selected_account = read_access.get_selected_account();

        let currency = selected_account.currency.as_currency_str().to_string();
        let balance = selected_account.balance.to_string();
        let is_live = selected_account.is_live;

        (currency, balance, is_live)
    };

    let total_phrase = LANG.total.as_str();

    let main_form_state = use_shared_state::<MainFormState>(cx).unwrap();
    render! {
        button {
            id: "balance-widget",
            class: "btn dropdown-toggle",

            onclick: move |_| {
                main_form_state.write().show_select_account();
            },
            div { style: "display:inline-block",
                div { style: "font-size: 14px;",
                    span { "{currency}" }
                    span { "{balance}" }
                    real_demo_badge { is_live: is_live }
                }
                div { class: "bottom-line",
                    span { style: "color: var(--label-color)", "{total_phrase}:" }
                    span { "{currency}" }
                    span { "{balance}" }
                }
            }
        }
    }
}
