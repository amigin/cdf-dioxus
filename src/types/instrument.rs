use my_nosql_contracts::TradingInstrumentNoSqlEntity;

use super::InstrumentId;

#[derive(Clone)]
pub struct Instrument {
    pub instrument_id: InstrumentId,
    pub name: String,
    pub digits: u32,
    pub group_id: Option<String>,
}

impl<'s> Into<Instrument> for &'s TradingInstrumentNoSqlEntity {
    fn into(self) -> Instrument {
        Instrument {
            instrument_id: self.get_id().into(),
            name: self.name.to_string(),
            digits: self.digits as u32,
            group_id: self.group_id.clone(),
        }
    }
}
