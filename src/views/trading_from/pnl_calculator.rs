use dioxus::prelude::*;

pub fn pnl_calculator(cx: Scope) -> Element {
    render! { button { id: "pnl-calc-btn", class: "btn btn-outline-dark", "Pnl Calculator" } }
}
