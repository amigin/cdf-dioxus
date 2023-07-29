use crate::types::InstrumentId;

pub struct FavInstrumentsState {
    selected: InstrumentId,
    instruments: Vec<InstrumentId>,
}

impl FavInstrumentsState {
    pub fn new() -> Self {
        Self {
            selected: "".into(),
            instruments: vec![],
        }
    }

    fn check_if_instrument_exists(&self, selected: &InstrumentId) -> bool {
        self.instruments
            .iter()
            .any(|itm| itm.as_str() == selected.as_str())
    }

    pub fn set_selected(&mut self, selected: InstrumentId) {
        if self.check_if_instrument_exists(&selected) {
            self.selected = selected;
            return;
        }
    }

    pub fn remove(&mut self, selected: InstrumentId) -> Vec<InstrumentId> {
        self.instruments
            .retain(|itm| itm.as_str() != selected.as_str());

        self.instruments.clone()
    }

    pub fn get_selected(&self) -> &InstrumentId {
        &self.selected
    }

    pub fn add(&mut self, instrument_id: InstrumentId) -> Vec<InstrumentId> {
        if self.check_if_instrument_exists(&instrument_id) {
            return self.instruments.clone();
        }

        self.instruments.push(instrument_id);
        self.instruments.clone()
    }

    pub fn get_instruments(&self) -> &[InstrumentId] {
        &self.instruments
    }

    pub fn set_fav_instruments(
        &mut self,
        instruments: Vec<InstrumentId>,
        selected: Option<InstrumentId>,
    ) {
        self.instruments = instruments;

        if selected.is_none() {
            if self.instruments.len() == 0 {
                self.selected = "".into();
                return;
            }

            self.selected = self.instruments[0].clone();
            return;
        }

        let selected = selected.unwrap();

        if self.check_if_instrument_exists(&selected) {
            self.selected = selected;
            return;
        }

        if self.instruments.len() == 0 {
            self.selected = "".into();
            return;
        }

        self.selected = self.instruments[0].clone();
    }
}
