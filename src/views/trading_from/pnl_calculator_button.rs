use dioxus::prelude::*;

use crate::{states::TradingPanelState, views::trading_from::pnl_calculator_panel};

pub fn pnl_calculator_button(cx: Scope) -> Element {
    let trading_panel_state = use_shared_state::<TradingPanelState>(cx).unwrap();
    let mut result = Vec::with_capacity(2);

    result.push(rsx! {
        button {
            id: "pnl-calc-btn",
            onclick: move |_| {
                trading_panel_state.write().toggle_show_pnl_calc();
            },
            class: "btn btn-outline-dark",
            "Pnl Calculator"
        }
    });

    if trading_panel_state.read().is_show_pnl_calc() {
        result.push(rsx! { pnl_calculator_panel {} })
    }

    render!(result.into_iter())
}
