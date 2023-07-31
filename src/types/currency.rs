#[derive(Clone)]
pub struct Currency(String);

impl Currency {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn as_currency_str(&self) -> &str {
        match self.0.as_str() {
            "USD" => "$",
            _ => &self.0,
        }
    }
}

impl Into<String> for Currency {
    fn into(self) -> String {
        self.0
    }
}

impl Into<Currency> for String {
    fn into(self) -> Currency {
        Currency::new(self)
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Currency:").field(&self.0).finish()
    }
}
