pub struct MainFormState {
    pub show_select_account: bool,
    pub sync_thread_is_on: bool,
}

impl MainFormState {
    pub fn new() -> Self {
        Self {
            show_select_account: false,
            sync_thread_is_on: false,
        }
    }

    pub fn has_dialog_pad(&self) -> bool {
        self.show_select_account
    }

    pub fn show_dialog_account(&mut self) {
        self.show_select_account = true;
    }

    pub fn hide_dialog(&mut self) {
        self.show_select_account = false;
    }
}
