use std::collections::HashMap;

use crate::types::InstrumentId;

pub enum TradingPanelModel {
    ShowTradingCalculator,
    SetPrice,
}

pub struct TradingPanelState {
    modals: Option<TradingPanelModel>,

    pub purchase_at_price: HashMap<String, String>,
}

impl TradingPanelState {
    pub fn new() -> Self {
        Self {
            modals: None,
            purchase_at_price: HashMap::new(),
        }
    }

    pub fn toggle_set_price(&mut self) {
        self.modals = Some(TradingPanelModel::SetPrice);
    }

    pub fn is_show_set_price(&self) -> bool {
        if let Some(TradingPanelModel::SetPrice) = self.modals {
            true
        } else {
            false
        }
    }

    pub fn hide_everything(&mut self) {
        self.modals = None;
    }

    pub fn get_purchase_at_price(&self, instrument_id: &InstrumentId) -> Option<&String> {
        self.purchase_at_price.get(instrument_id.as_str())
    }

    pub fn set_purchase_at_price(&mut self, instrument_id: &InstrumentId, price: String) {
        self.purchase_at_price
            .insert(instrument_id.as_str().to_string(), price);
    }

    pub fn reset_purchase_at(&mut self, instrument_id: &InstrumentId) {
        self.purchase_at_price.remove(instrument_id.as_str());
    }
}
