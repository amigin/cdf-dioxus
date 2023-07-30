use dioxus::prelude::*;

use crate::states::{GlobalState, MainFormState};

pub fn render_settings_menu(cx: Scope) -> Element {
    let main_form_state = use_shared_state::<MainFormState>(cx).unwrap();

    let mut security_class = "settings-menu-item";
    let mut balance_history_class = "settings-menu-item";
    let mut withdraw_class = "settings-menu-item";

    match main_form_state.read().get_settings_menu() {
        crate::states::SettingsMenuItem::Security => {
            security_class = "settings-menu-item-active";
        }
        crate::states::SettingsMenuItem::BalanceHistory => {
            balance_history_class = "settings-menu-item-active";
        }
        crate::states::SettingsMenuItem::Withdraw => {
            withdraw_class = "settings-menu-item-active";
        }
    }
    render! {
        div {
            class: "{security_class}",
            onclick: move |_| {
                main_form_state
                    .write()
                    .show_settings_form(crate::states::SettingsMenuItem::Security);
            },
            "Security"
        }
        div {
            class: "{withdraw_class}",
            onclick: move |_| {
                main_form_state
                    .write()
                    .show_settings_form(crate::states::SettingsMenuItem::Withdraw);
            },
            "Withdraw"
        }
        div {
            class: "{balance_history_class}",
            onclick: move |_| {
                main_form_state
                    .write()
                    .show_settings_form(crate::states::SettingsMenuItem::BalanceHistory);
            },
            "Balance history"
        }
        hr {}
        div {
            class: "settings-menu-item",
            onclick: move |_| {
                let global_state = use_shared_state::<GlobalState>(cx).unwrap();
                global_state.write().logout();
            },
            "Logout"
        }
    }
}
