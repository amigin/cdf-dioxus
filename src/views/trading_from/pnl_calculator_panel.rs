use dioxus::prelude::*;

use crate::{
    states::{BidAskSnapshotState, FavInstrumentsState, InstrumentsState, TradingPanelState},
    types::InstrumentId,
    views::{icons::*, widgets::faw_instruments::render_avatar},
};

pub fn pnl_calculator_panel(cx: Scope) -> Element {
    let is_buy = use_state(cx, || true);
    let value_invest = use_state(cx, || "1000");
    let value_multiplier = use_state(cx, || "5");

    let value_exit_price = use_state(cx, || "");

    let trading_panel_state = use_shared_state::<TradingPanelState>(cx).unwrap();

    let select_instrument = use_state(cx, || false);

    let instrument_id = use_shared_state::<FavInstrumentsState>(cx)
        .unwrap()
        .read()
        .get_selected()
        .clone();

    let selected_instrument_id = use_state(cx, || instrument_id);

    let instruments_state = use_shared_state::<InstrumentsState>(cx).unwrap();

    let (name, value_entry_price) =
        if let Some(instrument) = instruments_state.read().get(selected_instrument_id.get()) {
            let rate = use_shared_state::<BidAskSnapshotState>(cx)
                .unwrap()
                .read()
                .try_get_rate_as_str(Some(&instrument));

            let value_entry_price = if let Some(rate) = rate {
                use_state(cx, || rate)
            } else {
                use_state(cx, || "".to_string())
            };

            (instrument.name.clone(), value_entry_price)
        } else {
            (
                selected_instrument_id.get().as_str().to_string(),
                use_state(cx, || "".to_string()),
            )
        };

    let (buy_sell_toggle, btn_calc_class) = if *is_buy.get() {
        let result = rsx! {
            div { class: "btn-group", style: "width:100%;",
                button { class: "btn btn-success btn-sm", "Buy/Long" }
                button {
                    class: "btn btn-outline-dark btn-sm",
                    onclick: move |_| {
                        is_buy.set(false);
                    },
                    "Sell/Short"
                }
            }
        };

        (result, "btn-success")
    } else {
        let result = rsx! {
            div { class: "btn-group", style: "width:100%;",
                button {
                    class: "btn btn-outline-dark btn-sm",
                    onclick: move |_| {
                        is_buy.set(true);
                    },
                    "Buy/Long"
                }
                button { class: "btn btn-danger btn-sm", "Sell/Short" }
            }
        };

        (result, "btn-danger")
    };

    let content = if *select_instrument.get() {
        rsx! {
            render_select_instrument {
                selected_instrument_id: selected_instrument_id.get(),
                on_click: move |id| {
                    selected_instrument_id.set(id);
                    select_instrument.set(false);
                }
            }
        }
    } else {
        rsx! {
            div { style: "margin-top:10px", buy_sell_toggle }
            table { style: "width:100%; margin-top:10px;",
                tr {
                    td { style: "width:60%", "Invest" }
                    td { input {
                        r#type: "number",
                        class: "form-control form-control-sm",
                        value: "{value_invest.get()}"
                    } }
                }
                tr {
                    td { "Multiplier" }
                    td {
                        input {
                            r#type: "number",
                            class: "form-control form-control-sm",
                            value: "{value_multiplier.get()}"
                        }
                    }
                }
                tr {
                    td { "Multiplier" }
                    td {
                        input {
                            r#type: "number",
                            class: "form-control form-control-sm",
                            value: "{value_multiplier.get()}"
                        }
                    }
                }
                tr {
                    td { "Entry price" }
                    td {
                        input {
                            r#type: "number",
                            class: "form-control form-control-sm",
                            value: "{value_entry_price.get()}"
                        }
                    }
                }
                tr {
                    td { "Exit price" }
                    td {
                        input {
                            r#type: "number",
                            class: "form-control form-control-sm",
                            value: "{value_exit_price.get()}"
                        }
                    }
                }
                tr {
                    td { colspan: "2", hr {} }
                }
                tr {
                    td { "P/L" }
                    td { input { r#type: "number", class: "form-control form-control-sm", readonly: true } }
                }
                tr {
                    td { "P/L %" }
                    td { input { r#type: "number", class: "form-control form-control-sm", readonly: true } }
                }
                tr {
                    td { "Liquidation price" }
                    td { input { r#type: "number", class: "form-control form-control-sm", readonly: true } }
                }
            }
            div { style: "margin-top:20px",
                button { style: "width:100%", class: "btn {btn_calc_class} btn-sm", "Calculate" }
            }
        }
    };

    render! {
        div { id: "pnlCalculatorPanel",
            div { class: "floating-panel-header",
                div { class: "title", "Pnl calculator" }
                div {
                    class: "close-icon",
                    onclick: move |_| {
                        trading_panel_state.write().hide_everything();
                    },
                    close_icon {}
                }
            }

            div {
                label { "Select instrument" }
                div {
                    class: "select-instrument",
                    onclick: move |_| {
                        let value = *select_instrument.get();
                        select_instrument.set(!value);
                    },
                    div { render_avatar { id: selected_instrument_id.get().clone() } }
                    div { class: "instrument-name", name.as_str() }
                    div { style: "display: flex;align-items: center;padding-right: 5px;",
                        drop_down_icon {}
                    }
                }
            }
            content
        }
    }
}

#[inline_props]
fn render_select_instrument<'s>(
    cx: Scope<'s>,
    selected_instrument_id: &'s InstrumentId,
    on_click: EventHandler<'s, InstrumentId>,
) -> Element<'s> {
    let mut instruments = use_shared_state::<InstrumentsState>(cx)
        .unwrap()
        .read()
        .instruments
        .clone();

    instruments.remove(selected_instrument_id.as_str());

    let elements = instruments.values().map(|itm| {
        let itm = itm.clone();
        rsx! {
            div {
                style: "display:flex;align-items:center;padding:5px; cursor:pointer;",
                onclick: move |_| {
                    on_click.call(itm.instrument_id.clone());
                },

                div { style: "display:flex", render_avatar { id: itm.instrument_id.clone() } }
                div { style: "display:flex", itm.name.as_str() }
            }
        }
    });
    render! {
        div { style: "height: 400px;overflow-y: auto;", elements }
    }
}
