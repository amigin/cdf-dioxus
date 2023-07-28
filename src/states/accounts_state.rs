use crate::types::*;

pub struct AccountsStateInner {
    pub selected_account: AccountId,
    pub accounts: Vec<TraderAccount>,
}

pub struct AccountsState {
    pub inner: Option<AccountsStateInner>,
}

impl AccountsState {
    pub fn new() -> Self {
        Self { inner: None }
    }

    pub fn get_selected_account_id(&self) -> &AccountId {
        &self.inner.as_ref().unwrap().selected_account
    }

    pub fn set_accounts(&mut self, selected_account: AccountId, accounts: Vec<TraderAccount>) {
        self.inner = Some(AccountsStateInner {
            selected_account,
            accounts,
        });
    }
}
