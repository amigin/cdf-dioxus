#[derive(Clone, PartialEq)]
pub struct TraderId(String);

impl TraderId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Into<String> for TraderId {
    fn into(self) -> String {
        self.0
    }
}

impl Into<TraderId> for String {
    fn into(self) -> TraderId {
        TraderId::new(self)
    }
}

impl<'s> Into<TraderId> for &'s str {
    fn into(self) -> TraderId {
        TraderId::new(self.to_string())
    }
}

impl std::fmt::Display for TraderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for TraderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TraderId:").field(&self.0).finish()
    }
}
