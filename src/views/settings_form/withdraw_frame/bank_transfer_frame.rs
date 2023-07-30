use dioxus::prelude::*;

pub fn bank_transfer_frame(cx: Scope) -> Element {
    render! {
        div { style: "margin-top:20px", label { "Withdrawable" } }

        div { style: "font-size: 24px;",
            "USD: "
            span { style: "font-weight:bold", "0.00" }
        }

        div { style: "margin-top:20px", label { "Payment methods" } }

        div { class: "withdraw-methods",
            div { class: "withdraw-method withdraw-method-active",

                div { class: "withdraw-method-content",
                    div { img { class: "withdraw-method-icon", src: "/img/usdt.svg" } }
                    div { class: "name", "Tether: USDT" }
                }
            }
        }

        div { class: "withdraw-data",
            div { class: "withdraw-input_group",
                label { class: "form-label", "Amount" }
                input { class: "form-control", r#type: "number", placeholder: "Type amount" }
            }

            div { class: "withdraw-input_group",
                label { class: "form-label", "TRX wallet address" }
                input { class: "form-control", placeholder: "Trx wallet address" }
            }
        }

        div { button { class: "btn btn-success", style: "margin-top: 20px;", "Withdraw" } }
    }
}
