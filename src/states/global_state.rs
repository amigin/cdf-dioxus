use crate::types::TraderId;

pub enum GlobalState {
    NonAuthenticated,
    ResetPassword,
    SignUp,
    Loading { trader_id: TraderId, email: String },
    Authenticated { trader_id: TraderId },
}

impl GlobalState {
    pub fn as_ref(&self) -> &Self {
        self
    }

    pub fn set_loading(&mut self, trader_id: TraderId, email: String) {
        *self = Self::Loading { trader_id, email }
    }

    pub fn set_reset_password(&mut self) {
        *self = Self::ResetPassword;
    }

    pub fn set_authenticated(&mut self) {
        match self {
            Self::Loading { trader_id, .. } => {
                *self = Self::Authenticated {
                    trader_id: trader_id.clone(),
                }
            }
            _ => panic!("GlobalState::set_authenticated() called on non-loading state"),
        }
    }

    pub fn set_sign_up(&mut self) {
        *self = Self::SignUp;
    }

    pub fn set_login(&mut self) {
        *self = Self::NonAuthenticated;
    }

    pub fn get_trader_id(&self) -> &TraderId {
        match self {
            Self::Authenticated { trader_id } => trader_id,
            Self::Loading { trader_id, .. } => trader_id,
            _ => panic!("GlobalState::get_trader_id() called on non-authenticated state"),
        }
    }

    pub fn get_email(&self) -> String {
        match self {
            Self::Loading { email, .. } => email.clone(),
            _ => panic!("Email can be retrieved only from loading state"),
        }
    }

    pub fn logout(&mut self) {
        *self = Self::NonAuthenticated;
    }
}
