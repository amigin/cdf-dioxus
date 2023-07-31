use std::collections::HashMap;

use crate::types::InstrumentId;

pub enum TradingPanelModel {
    ShowTradingCalculator,
    SetAutoClose,
    SetPrice,
}

#[derive(Clone, Copy)]
pub enum ProdValueMode {
    Currency,
    Percent,
}

impl ProdValueMode {
    pub fn get_sign(&self) -> &'static str {
        match self {
            ProdValueMode::Currency => "$",
            ProdValueMode::Percent => "%",
        }
    }

    pub fn into_value(&self, value: String) -> TpSlValue {
        match self {
            ProdValueMode::Currency => TpSlValue::Currency(value),
            ProdValueMode::Percent => TpSlValue::Percent(value),
        }
    }
}

#[derive(Clone, Debug)]
pub enum TpSlValue {
    Currency(String),
    Percent(String),
}

impl TpSlValue {
    pub fn update(&self, value: String) -> Self {
        match self {
            &Self::Currency(_) => Self::Currency(value),
            &Self::Percent(_) => Self::Percent(value),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Currency(value) => value.as_str(),
            Self::Percent(value) => value.as_str(),
        }
    }

    pub fn to_panel_string(&self, currency: &str) -> String {
        match self {
            Self::Currency(value) => format!("{}{}", currency, value),
            Self::Percent(value) => format!("{}%", value),
        }
    }
}

pub struct TradingPanelState {
    modals: Option<TradingPanelModel>,

    pub profit_value_mode: ProdValueMode,
    pub loss_value_mode: ProdValueMode,

    pub purchase_at_price: HashMap<String, String>,

    pub tp: HashMap<String, TpSlValue>,
    pub sl: HashMap<String, TpSlValue>,
}

impl TradingPanelState {
    pub fn new() -> Self {
        Self {
            modals: None,
            purchase_at_price: HashMap::new(),
            tp: HashMap::new(),
            sl: HashMap::new(),
            profit_value_mode: ProdValueMode::Currency,
            loss_value_mode: ProdValueMode::Currency,
        }
    }

    pub fn toggle_tp_mode(&mut self) {
        self.profit_value_mode = match self.profit_value_mode {
            ProdValueMode::Currency => ProdValueMode::Percent,
            ProdValueMode::Percent => ProdValueMode::Currency,
        }
    }

    pub fn toggle_sl_mode(&mut self) {
        self.loss_value_mode = match self.loss_value_mode {
            ProdValueMode::Currency => ProdValueMode::Percent,
            ProdValueMode::Percent => ProdValueMode::Currency,
        }
    }

    pub fn toggle_set_price(&mut self) {
        self.modals = Some(TradingPanelModel::SetPrice);
    }

    pub fn toggle_set_auto_close(&mut self) {
        self.modals = Some(TradingPanelModel::SetAutoClose);
    }

    pub fn is_show_set_price(&self) -> bool {
        if let Some(TradingPanelModel::SetPrice) = self.modals {
            true
        } else {
            false
        }
    }

    pub fn is_show_set_auto_close(&self) -> bool {
        if let Some(TradingPanelModel::SetAutoClose) = self.modals {
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

    pub fn set_tp(&mut self, instrument_id: &InstrumentId, value: Option<TpSlValue>) {
        match value {
            Some(value) => {
                self.tp.insert(instrument_id.as_str().to_string(), value);
            }
            None => {
                self.tp.remove(instrument_id.as_str());
            }
        }
    }

    pub fn set_sl(&mut self, instrument_id: &InstrumentId, value: Option<TpSlValue>) {
        match value {
            Some(value) => {
                self.sl.insert(instrument_id.as_str().to_string(), value);
            }
            None => {
                self.sl.remove(instrument_id.as_str());
            }
        }
    }

    pub fn get_tp_or_sl(
        &self,
        instrument_id: &InstrumentId,
    ) -> Option<(Option<TpSlValue>, Option<TpSlValue>)> {
        let tp = if let Some(value) = self.tp.get(instrument_id.as_str()) {
            Some(value.clone())
        } else {
            None
        };

        let sl = if let Some(value) = self.sl.get(instrument_id.as_str()) {
            Some(value.clone())
        } else {
            None
        };

        if tp.is_some() || sl.is_some() {
            return Some((tp, sl));
        }

        None
    }
}
