pub enum ShowMenu {
    SelectAccount,
    ShowProfileMenu,
    SelectInstrument,
}

pub struct MainFormState {
    pub sync_thread_is_on: bool,

    pub show_menu: Option<ShowMenu>,
}

impl MainFormState {
    pub fn new() -> Self {
        Self {
            sync_thread_is_on: false,
            show_menu: None,
        }
    }

    pub fn select_instrument_is_shown(&self) -> bool {
        match &self.show_menu {
            Some(ShowMenu::SelectInstrument) => true,
            _ => false,
        }
    }

    pub fn profile_menu_is_shown(&self) -> bool {
        match &self.show_menu {
            Some(ShowMenu::ShowProfileMenu) => true,
            _ => false,
        }
    }

    pub fn select_account_is_shown(&self) -> bool {
        match &self.show_menu {
            Some(ShowMenu::SelectAccount) => true,
            _ => false,
        }
    }

    pub fn hide_dialog(&mut self) {
        self.show_menu = None;
    }

    pub fn show_select_account(&mut self) {
        self.show_menu = Some(ShowMenu::SelectAccount);
    }

    pub fn show_select_instrument(&mut self) {
        self.show_menu = Some(ShowMenu::SelectInstrument);
    }

    pub fn click_profile_menu(&mut self) {
        if self.profile_menu_is_shown() {
            self.show_menu = None;
        } else {
            self.show_menu = Some(ShowMenu::ShowProfileMenu);
        }
    }
}
