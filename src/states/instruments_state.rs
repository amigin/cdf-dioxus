use std::{collections::HashMap, sync::Arc};

use my_nosql_contracts::TradingInstrumentNoSqlEntity;

use crate::types::Instrument;

pub struct InstrumentsState {
    pub instruments: HashMap<String, Instrument>,
}

impl InstrumentsState {
    pub fn new() -> Self {
        Self {
            instruments: HashMap::new(),
        }
    }

    pub fn set_instruments(&mut self, src: Vec<Arc<TradingInstrumentNoSqlEntity>>) {
        self.instruments.clear();

        for itm in src {
            self.instruments
                .insert(itm.get_id().to_string(), itm.as_ref().into());
        }
    }
}
