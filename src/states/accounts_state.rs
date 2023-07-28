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

    pub fn get_selected_account(&self) -> &TraderAccount {
        let selected_account_id = self.get_selected_account_id();
        let accounts = &self.inner.as_ref().unwrap().accounts;
        accounts
            .iter()
            .find(|x| x.account_id == *selected_account_id)
            .unwrap()
    }

    pub fn set_selected_account(&mut self, selected_account: AccountId) {
        self.inner.as_mut().unwrap().selected_account = selected_account;
    }

    pub fn set_accounts(&mut self, selected_account: AccountId, accounts: Vec<TraderAccount>) {
        self.inner = Some(AccountsStateInner {
            selected_account,
            accounts,
        });
    }

    pub fn get_accounts(&self) -> &[TraderAccount] {
        &self.inner.as_ref().unwrap().accounts
    }
}
