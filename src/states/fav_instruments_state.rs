use crate::types::InstrumentId;

pub struct FavInstrumentsState {
    pub selected: InstrumentId,
    pub instruments: Vec<InstrumentId>,
}

impl FavInstrumentsState {
    pub fn new() -> Self {
        Self {
            selected: "".into(),
            instruments: vec![],
        }
    }
}
