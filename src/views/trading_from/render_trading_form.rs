use dioxus::prelude::*;

use crate::views::{trading_from::trading_panel, widgets::faw_instruments::fav_instruments_widget};

pub fn render_trading_form(cx: Scope) -> Element {
    render! {
        div { id: "fav-instruments", fav_instruments_widget {} }

        div { id: "trading-panel",
            table { class: "tech-table", style: "width:100%; height:100%;",
                tr {
                    td { trading_panel {} }
                }
            }
        }
    }
}
