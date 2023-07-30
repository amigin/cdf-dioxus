use dioxus::prelude::*;

use crate::{
    states::{MainFormState, SettingsMenuItem},
    views::{icons::close_settings_panel_icon, settings_form::*},
};

pub fn render_settings_form(cx: Scope) -> Element {
    let main_form_state = use_shared_state::<MainFormState>(cx).unwrap();

    let (caption, widget) = match main_form_state.read().get_settings_menu() {
        SettingsMenuItem::Security => ("Security settings", rsx!(change_password_frame {})),
        SettingsMenuItem::BalanceHistory => ("Balance history", rsx!(balance_history_frame {})),
        SettingsMenuItem::Withdraw => ("Withdraw", rsx!(withdraw_frame {})),
    };

    render! {
        div { style: "padding: 10px; padding-left: 20px;",
            div {
                style: "text-align: right; cursor: pointer;",
                onclick: |_| {
                    main_form_state.write().hide_dialog();
                },
                close_settings_panel_icon {}
            }
            table { style: "text-align: left; ",
                tr {
                    td { style: "width: 150px;",
                        h3 { b { "Account" } }
                    }
                    td { div { width: "100px" } }
                    td { style: "width: 400px;",
                        h3 {
                            b { caption }
                        }
                    }
                }
                tr {
                    td { style: "vertical-align: top;",
                        div { style: "height:20px" }
                        render_settings_menu {}
                    }
                    td {}
                    td { style: "vertical-align: top;",
                        div {
                            div { style: "height:20px" }
                            widget
                        }
                    }
                }
            }
        }
    }
}
