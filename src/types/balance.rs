#[derive(Clone, Copy)]
pub struct Balance(f64);

impl Balance {
    pub fn new(id: f64) -> Self {
        Self(id)
    }

    pub fn get_value(&self) -> f64 {
        self.0
    }

    pub fn to_string(&self) -> String {
        format!("{:.2}", self.0)
    }
}

impl Into<f64> for Balance {
    fn into(self) -> f64 {
        self.0
    }
}

impl Into<Balance> for f64 {
    fn into(self) -> Balance {
        Balance::new(self)
    }
}

impl std::fmt::Display for Balance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for Balance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Balance:").field(&self.0).finish()
    }
}
