#[derive(Clone)]
pub struct InstrumentId(String);

impl InstrumentId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }

    pub fn equals_to(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Into<String> for InstrumentId {
    fn into(self) -> String {
        self.0
    }
}

impl Into<InstrumentId> for String {
    fn into(self) -> InstrumentId {
        InstrumentId::new(self)
    }
}

impl<'s> Into<InstrumentId> for &'s str {
    fn into(self) -> InstrumentId {
        InstrumentId::new(self.to_string())
    }
}
impl std::fmt::Display for InstrumentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for InstrumentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("InstrumentId:").field(&self.0).finish()
    }
}
