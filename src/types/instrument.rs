use my_nosql_contracts::TradingInstrumentNoSqlEntity;

use super::InstrumentId;

#[derive(Clone)]
pub struct Instrument {
    pub instrument_id: InstrumentId,
    pub name: String,
}

impl<'s> Into<Instrument> for &'s TradingInstrumentNoSqlEntity {
    fn into(self) -> Instrument {
        Instrument {
            instrument_id: self.get_id().into(),
            name: self.name.to_string(),
        }
    }
}
