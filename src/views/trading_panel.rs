use dioxus::prelude::*;
pub fn trading_panel(cx: Scope) -> Element {
    render! {
        button { id: "pnl-calc-btn", class: "btn btn-outline-dark", "Pnl Calculator" }

        div {
            label { "Invest" }
            input {
                r#type: "number",
                class: "form-control",
                id: "investAmount",
                placeholder: "Invest"
            }
        }

        div {
            label { "Multiplier" }
            input {
                r#type: "text",
                class: "form-control",
                id: "investAmount",
                placeholder: "Multiplier",
                value: "x15"
            }
        }

        div {
            label { "Auto close" }
            button { id: "btnAutoClose", class: "btn btn-outline-dark", "Set" }
        }

        table { class: "tech-table", style: "width:100%;",
            tr {
                td { style: "color:gray", "Volume" }
                td { style: "text-align: right;", "USD: 250" }
            }
            tr {
                td { style: "color:gray", "Spread" }
                td { style: "text-align: right;", "0.01" }
            }
        }

        button { id: "btnBuy", class: "btn btn-success btn-lg", "Buy" }
        button { id: "btnSell", class: "btn btn-danger btn-lg", "Sell" }

        div {
            label { id: "labelSetPurchaseAt", "Purchase at" }
            button { id: "btnSetPurchaseAt", class: "btn btn-outline-dark", "Set Price" }
        }
    }
}
