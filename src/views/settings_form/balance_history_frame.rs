use dioxus::prelude::*;

use crate::views::icons::*;

pub fn balance_history_frame(cx: Scope) -> Element {
    let table_lines = (0..6).into_iter().map(|_| {
        rsx! {
            tr { class: "table-line",
                td { div { "30 Jul 2023, 10:29:12" } }
                td { style: "color:var(--balance-loss-color)", "-USD:100.00" }
                td { "USD:0.00" }
                td { style: "color: var(--label-color)", "BalanceCorrection" }
            }
        }
    });

    render! {
        div { label { class: "form-label", "Period" } }
        div { style: "width: 220px;", class: "input-group",
            span {
                style: "background-color: var(--vz-input-bg-custom)",
                class: "input-group-text",
                calendar_settings_panel_icon {}
            }

            select { class: "form-select from-select-sm", style: "border-left:none",
                option { "Today" }
                option { "Week" }
                option { "Month" }
                option { "Year" }
            }
        }

        div { style: "margin-top: 20px;",

            table { style: "width:1024px", class: "table table-history",

                tr { class: "table-header",
                    th { "Date" }
                    th { "Amount" }
                    th { "Balance" }
                    th { "Description" }
                }

                table_lines
            }
        }
    }
}
