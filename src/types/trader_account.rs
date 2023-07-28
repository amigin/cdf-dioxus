use super::*;

#[derive(Clone)]
pub struct TraderAccount {
    pub account_id: AccountId,
    pub currency: Currency,
    pub balance: Balance,
    pub trading_disabled: bool,
    pub is_live: bool,
}

impl TraderAccount {
    pub fn is_same_account(&self, other: &Self) -> bool {
        self.account_id.as_str() == other.account_id.as_str()
    }
}
