use super::pnl_calculator_panel;
use crate::{lang::LANG, states::TradingPanelState};
use dioxus::prelude::*;

pub fn pnl_calculator_button(cx: Scope) -> Element {
    let trading_panel_state = use_shared_state::<TradingPanelState>(cx).unwrap();
    let mut result = Vec::with_capacity(2);

    let pnl_calc_lang = &LANG.pnl_calculator;

    result.push(rsx! {
        button {
            id: "pnl-calc-btn",
            onclick: move |_| {
                trading_panel_state.write().toggle_show_pnl_calc();
            },
            class: "btn btn-outline-dark",
            "{pnl_calc_lang.pnl_calculator_button}"
        }
    });

    if trading_panel_state.read().is_show_pnl_calc() {
        result.push(rsx! { pnl_calculator_panel {} })
    }

    render!(result.into_iter())
}
