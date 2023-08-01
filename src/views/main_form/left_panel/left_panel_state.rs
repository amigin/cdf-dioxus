pub enum LeftPanelState {
    Nothing,
    ShowMarkets,
    ShowPortfolio,
    ShowHistory,
}

impl LeftPanelState {
    pub fn new() -> Self {
        LeftPanelState::Nothing
    }

    pub fn is_panel_shown(&self) -> bool {
        match self {
            LeftPanelState::Nothing => false,
            _ => true,
        }
    }

    pub fn is_show_markets(&self) -> bool {
        match self {
            LeftPanelState::ShowMarkets => true,
            _ => false,
        }
    }

    pub fn toggle_show_markets(&self) -> Self {
        if self.is_show_markets() {
            LeftPanelState::Nothing
        } else {
            LeftPanelState::ShowMarkets
        }
    }

    pub fn is_show_portfolio(&self) -> bool {
        match self {
            LeftPanelState::ShowPortfolio => true,
            _ => false,
        }
    }

    pub fn toggle_show_portfolio(&self) -> Self {
        if self.is_show_portfolio() {
            LeftPanelState::Nothing
        } else {
            LeftPanelState::ShowPortfolio
        }
    }

    pub fn is_show_history(&self) -> bool {
        match self {
            LeftPanelState::ShowHistory => true,
            _ => false,
        }
    }

    pub fn toggle_show_history(&self) -> Self {
        if self.is_show_history() {
            LeftPanelState::Nothing
        } else {
            LeftPanelState::ShowHistory
        }
    }
}
