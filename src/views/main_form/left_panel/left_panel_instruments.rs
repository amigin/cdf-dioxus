use dioxus::prelude::*;

use crate::{
    states::InstrumentsState,
    views::widgets::faw_instruments::{render_avatar, render_rate},
};

pub fn left_panel_instruments(cx: Scope) -> Element {
    let the_state = use_state(cx, || LeftPanelInstrumentsState::new());

    let instruments = use_shared_state::<InstrumentsState>(cx).unwrap();

    let mut stocks_active = "";
    let mut fx_active = "";
    let mut indices_active = "";
    let mut crypto_active = "";
    let mut commodities_active = "";

    let active_tab = the_state.get().clone();

    match the_state.get() {
        LeftPanelInstrumentsState::Stock => {
            stocks_active = "active";
        }
        LeftPanelInstrumentsState::Fx => {
            fx_active = "active";
        }
        LeftPanelInstrumentsState::Indices => {
            indices_active = "active";
        }
        LeftPanelInstrumentsState::Crypto => {
            crypto_active = "active";
        }
        LeftPanelInstrumentsState::Commodities => {
            commodities_active = "active";
        }
    }

    let instruments = instruments.read();
    let instruments = instruments
        .instruments
        .values()
        .filter(|itm| active_tab.is_mine(itm.group_id.as_ref()))
        .map(|itm| {
            rsx! {
                div { class: "instrument",
                    div { class: "icon", render_avatar { id: itm.instrument_id.clone() } }
                    div { class: "name",
                        div { style: "display:block",
                            div { style: "display:block", "{itm.name.as_str()}" }
                            div { style: "display:block", class: "id", itm.instrument_id.as_str() }
                        }
                    }
                    div { class: "rate", render_rate { instrument_id: itm.instrument_id.clone() } }
                }
            }
        });

    render! {
        div { id: "markets-menu",
            div {
                class: "item {stocks_active}",
                onclick: |_| {
                    the_state.set(LeftPanelInstrumentsState::Stock);
                },
                "Stocks"
            }
            div {
                class: "item {fx_active}",
                onclick: |_| {
                    the_state.set(LeftPanelInstrumentsState::Fx);
                },
                "FX"
            }
            div {
                class: "item {indices_active}",
                onclick: |_| {
                    the_state.set(LeftPanelInstrumentsState::Indices);
                },
                "Indices"
            }
            div {
                class: "item {crypto_active}",
                onclick: |_| {
                    the_state.set(LeftPanelInstrumentsState::Crypto);
                },
                "Crypto"
            }
            div {
                class: "item {commodities_active}",
                onclick: |_| {
                    the_state.set(LeftPanelInstrumentsState::Commodities);
                },
                "Commodities"
            }
        }

        div { id: "markets-content", instruments }
    }
}

#[derive(Clone)]
pub enum LeftPanelInstrumentsState {
    Stock,
    Fx,
    Indices,
    Crypto,
    Commodities,
}

impl LeftPanelInstrumentsState {
    pub fn new() -> Self {
        LeftPanelInstrumentsState::Stock
    }

    pub fn is_mine(&self, name: Option<&String>) -> bool {
        if name.is_none() {
            return false;
        }

        let name = name.unwrap();

        match self {
            LeftPanelInstrumentsState::Stock => name.starts_with("STOCK"),
            LeftPanelInstrumentsState::Fx => name.starts_with("FOREX"),
            LeftPanelInstrumentsState::Indices => {
                name.starts_with("INDICES") || name.starts_with("INDICIES")
            }
            LeftPanelInstrumentsState::Crypto => name.starts_with("CRYPTO"),
            LeftPanelInstrumentsState::Commodities => name.starts_with("COMMODITIES"),
        }
    }
}
