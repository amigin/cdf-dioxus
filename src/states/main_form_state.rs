pub enum ShowMenu {
    SelectAccount,
    ShowProfileMenu,
    SelectInstrument,
}

impl ShowMenu {
    pub fn select_instrument_is_shown(&self) -> bool {
        match &self {
            ShowMenu::SelectInstrument => true,
            _ => false,
        }
    }

    pub fn profile_menu_is_shown(&self) -> bool {
        match &self {
            ShowMenu::ShowProfileMenu => true,
            _ => false,
        }
    }

    pub fn select_account_is_shown(&self) -> bool {
        match &self {
            ShowMenu::SelectAccount => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy)]
pub enum SettingsMenuItem {
    Security,
    BalanceHistory,
    Withdraw,
}

pub enum MainFormMode {
    MainForm(Option<ShowMenu>),
    SettingsForm(SettingsMenuItem),
}

pub struct MainFormState {
    pub sync_thread_is_on: bool,
    pub mode: MainFormMode,
}

impl MainFormState {
    pub fn new() -> Self {
        Self {
            sync_thread_is_on: false,
            mode: MainFormMode::MainForm(None),
        }
    }

    fn unwrap_main_as_bool(&self, is_my_menu: impl Fn(&ShowMenu) -> bool) -> bool {
        match &self.mode {
            MainFormMode::MainForm(show_menu) => {
                if let Some(show_menu) = show_menu {
                    is_my_menu(show_menu)
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn select_instrument_is_shown(&self) -> bool {
        self.unwrap_main_as_bool(|show_menu| show_menu.select_instrument_is_shown())
    }

    pub fn profile_menu_is_shown(&self) -> bool {
        self.unwrap_main_as_bool(|show_menu| show_menu.profile_menu_is_shown())
    }

    pub fn select_account_is_shown(&self) -> bool {
        self.unwrap_main_as_bool(|show_menu| show_menu.select_account_is_shown())
    }

    pub fn hide_dialog(&mut self) {
        self.mode = MainFormMode::MainForm(None);
    }

    pub fn show_select_account(&mut self) {
        self.mode = MainFormMode::MainForm(Some(ShowMenu::SelectAccount));
    }

    pub fn show_select_instrument(&mut self) {
        self.mode = MainFormMode::MainForm(Some(ShowMenu::SelectInstrument));
    }

    pub fn is_main_form(&self) -> bool {
        match &self.mode {
            MainFormMode::MainForm(_) => true,
            MainFormMode::SettingsForm(_) => false,
        }
    }

    pub fn click_profile_menu(&mut self) {
        match &self.mode {
            MainFormMode::MainForm(mode) => {
                if let Some(show_menu) = mode {
                    if show_menu.profile_menu_is_shown() {
                        self.mode = MainFormMode::MainForm(None);
                        return;
                    }
                }
            }
            _ => {}
        }
        self.mode = MainFormMode::MainForm(Some(ShowMenu::ShowProfileMenu));
    }

    pub fn show_settings_form(&mut self, settings_item: SettingsMenuItem) {
        self.mode = MainFormMode::SettingsForm(settings_item);
    }

    pub fn get_settings_menu(&self) -> SettingsMenuItem {
        match &self.mode {
            MainFormMode::MainForm(_) => {
                panic!("We are in a main form")
            }
            MainFormMode::SettingsForm(settings) => settings.clone(),
        }
    }
}
