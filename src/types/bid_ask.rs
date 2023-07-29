use my_nosql_contracts::BidAskSnapshotNoSqlEntity;

use super::InstrumentId;

#[derive(Clone)]
pub struct BidAsk {
    pub instrument_id: InstrumentId,
    pub bid: f64,
    pub ask: f64,
}

impl BidAsk {
    pub fn is_same_with(&self, other: &Self) -> bool {
        self.bid == other.bid && self.ask == other.ask
    }
}

impl Into<BidAsk> for BidAskSnapshotNoSqlEntity {
    fn into(self) -> BidAsk {
        BidAsk {
            instrument_id: self.row_key.into(),
            bid: self.bid,
            ask: self.ask,
        }
    }
}

impl<'s> Into<BidAsk> for &'s BidAskSnapshotNoSqlEntity {
    fn into(self) -> BidAsk {
        BidAsk {
            instrument_id: self.row_key.as_str().into(),
            bid: self.bid,
            ask: self.ask,
        }
    }
}
