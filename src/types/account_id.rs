use std::fmt::Display;
#[derive(Clone, PartialEq)]
pub struct AccountId(String);

impl AccountId {
    pub fn new(id: String) -> Self {
        Self(id)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Into<AccountId> for String {
    fn into(self) -> AccountId {
        AccountId::new(self)
    }
}

impl Into<String> for AccountId {
    fn into(self) -> String {
        self.0
    }
}

impl Display for AccountId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for AccountId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AccountId:").field(&self.0).finish()
    }
}
