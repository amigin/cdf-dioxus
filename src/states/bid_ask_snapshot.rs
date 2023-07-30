use std::collections::HashMap;

use crate::types::{BidAsk, Instrument, InstrumentId};

pub struct BidAskSnapshotState {
    pub bid_ask: HashMap<String, BidAsk>,
    pub thread_is_taken: bool,
}

impl BidAskSnapshotState {
    pub fn new() -> Self {
        Self {
            bid_ask: HashMap::new(),
            thread_is_taken: false,
        }
    }

    pub fn update<'s>(&mut self, items: Vec<BidAsk>) {
        for item in items {
            self.bid_ask
                .insert(item.instrument_id.as_str().to_string(), item);
        }
    }

    pub fn get_rate_as_str(&self, instrument: Option<&Instrument>, id: &InstrumentId) -> String {
        //todo!("WE take Bid only for now")

        match instrument {
            Some(instrument) => {
                if let Some(bid_ask) = self.bid_ask.get(instrument.instrument_id.as_str()) {
                    return format!("{}", precision_f64(bid_ask.bid, instrument.digits));
                }
                let mut result = String::with_capacity(24);
                result.push_str("-.");

                for _ in 0..instrument.digits {
                    result.push('-');
                }

                result
            }
            None => {
                if let Some(bid_ask) = self.bid_ask.get(id.as_str()) {
                    return format!("{:.5}", bid_ask.bid);
                }

                "-.-----".to_string()
            }
        }
    }

    pub fn try_get_rate_as_str(&self, instrument: Option<&Instrument>) -> Option<String> {
        let instrument = instrument?;

        if let Some(bid_ask) = self.bid_ask.get(instrument.instrument_id.as_str()) {
            return format!("{}", precision_f64(bid_ask.bid, instrument.digits)).into();
        }

        None
    }
}

fn precision_f64(x: f64, decimals: u32) -> f64 {
    if x == 0. || decimals == 0 {
        0.
    } else {
        let shift = decimals as i32 - x.abs().log10().ceil() as i32;
        let shift_factor = 10_f64.powi(shift);

        (x * shift_factor).round() / shift_factor
    }
}
