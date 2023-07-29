use std::{collections::BTreeMap, sync::Arc};

use my_nosql_contracts::TradingInstrumentNoSqlEntity;

use crate::types::{Instrument, InstrumentId};

pub struct InstrumentsState {
    pub instruments: BTreeMap<String, Instrument>,
}

impl InstrumentsState {
    pub fn new() -> Self {
        Self {
            instruments: BTreeMap::new(),
        }
    }

    pub fn set_instruments(&mut self, src: Vec<Arc<TradingInstrumentNoSqlEntity>>) {
        self.instruments.clear();

        for itm in src {
            self.instruments
                .insert(itm.get_id().to_string(), itm.as_ref().into());
        }
    }

    pub fn get_name(&self, id: &InstrumentId) -> String {
        let instrument = self.instruments.get(id.as_str());

        if instrument.is_none() {
            return id.to_string();
        }

        let instrument = instrument.unwrap();

        instrument.name.clone()
    }
    pub fn get(&self, id: &InstrumentId) -> Option<&Instrument> {
        self.instruments.get(id.as_str())
    }
}
