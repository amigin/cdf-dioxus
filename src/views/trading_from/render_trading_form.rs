use dioxus::prelude::*;

use crate::views::{trading_from::trading_panel, widgets::faw_instruments::fav_instruments_widget};

pub fn render_fav_instruments_bar(cx: Scope) -> Element {
    render! {
        div { id: "fav-instruments", fav_instruments_widget {} }
    }
}

pub fn render_trading_panel(cx: Scope) -> Element {
    render! {
        div { id: "trading-panel",
            table { class: "tech-table", style: "width:100%; height:100%;",
                tr {
                    td { trading_panel {} }
                }
            }
        }
    }
}
