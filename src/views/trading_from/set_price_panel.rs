use dioxus::prelude::*;

use crate::{
    states::{BidAskSnapshotState, FavInstrumentsState, InstrumentsState, TradingPanelState},
    views::icons::close_icon,
};

pub fn render_set_price_panel(cx: Scope) -> Element {
    let rate_value = use_state(cx, || "".to_string());
    let trading_panel_state = use_shared_state::<TradingPanelState>(cx).unwrap();

    let rate = use_shared_state::<BidAskSnapshotState>(cx).unwrap();

    let fav_instruments_state = use_shared_state::<FavInstrumentsState>(cx).unwrap();

    let instrument_id = fav_instruments_state.read().get_selected().clone();

    let instrument_id2 = instrument_id.clone();

    let instruments_state = use_shared_state::<InstrumentsState>(cx).unwrap().read();

    let instrument = instruments_state.get(&instrument_id);

    let rate = if let Some(rate) = rate.read().try_get_rate_as_str(instrument) {
        let new_rate = rate.clone();
        rsx! {
            div { class: "rate", onclick: move |_| { rate_value.set(new_rate.clone()) }, rate }
        }
    } else {
        rsx! { div {} }
    };

    let apply_disabled = if rate_value.get().trim().len() > 0 {
        "false"
    } else {
        "true"
    };

    let mut result = Vec::with_capacity(2);

    /*  */

    if let Some(purchase_at) = trading_panel_state
        .read()
        .get_purchase_at_price(&instrument_id)
        .cloned()
    {
        result.push(rsx! {
            div { id: "purchaseAtPanel",
                div { class: "purchase-price", "{purchase_at}" }
                div {
                    class: "remove-button",
                    onclick: move |_| {
                        trading_panel_state.write().reset_purchase_at(&instrument_id);
                    },
                    "remove"
                }
            }
        });
    } else {
        result.push(rsx! {
            button {
                id: "btnSetPurchaseAt",
                onclick: move |_| {
                    rate_value.set("".to_string());
                    trading_panel_state.write().toggle_set_price();
                },
                class: "btn btn-outline-dark",
                "Set Price"
            }
        });
    }

    if trading_panel_state.read().is_show_set_price() {
        result.push(rsx! {
            div { id: "setPricePanel",
                div { class: "header",
                    div { class: "title", "Purchase at" }
                    div {
                        class: "close-icon",
                        onclick: move |_| {
                            trading_panel_state.write().hide_everything();
                        },
                        close_icon {}
                    }
                }

                div { style: "margin-top: 15px",
                    label { "When price is:" }
                    input {
                        class: "form-control form-control-sm",
                        value: "{rate_value}",
                        oninput: move |e| {
                            rate_value.set(e.value.clone());
                        }
                    }
                }

                div { style: "margin-top: 15px",
                    table { style: "width: 100%",
                        tr {
                            td { "Current price" }
                            td { style: "text-align:right", rate }
                        }
                    }
                }

                div { style: "margin-top: 20px",
                    button {
                        class: "btn btn-success btn-sm",
                        disabled: apply_disabled,
                        onclick: move |_| {
                            let price = rate_value.get().to_string();
                            trading_panel_state.write().set_purchase_at_price(&instrument_id2, price);
                            trading_panel_state.write().hide_everything();
                        },
                        style: "width:100%",
                        "Apply"
                    }
                }
            }
        });
    }

    render! {result.into_iter()}
}
