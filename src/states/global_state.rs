use crate::types::TraderId;

pub enum GlobalState {
    NonAuthenticated,
    SignUp,
    Loading { trader_id: TraderId },
    Authenticated { trader_id: TraderId },
}

impl GlobalState {
    pub fn as_ref(&self) -> &Self {
        self
    }

    pub fn set_loading(&mut self, trader_id: TraderId) {
        *self = Self::Loading { trader_id }
    }

    pub fn set_authenticated(&mut self) {
        match self {
            Self::Loading { trader_id } => {
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

    pub fn set_root(&mut self) {
        *self = Self::NonAuthenticated;
    }

    pub fn get_trader_id(&self) -> &TraderId {
        match self {
            Self::Authenticated { trader_id } => trader_id,
            Self::Loading { trader_id } => trader_id,
            _ => panic!("GlobalState::get_trader_id() called on non-authenticated state"),
        }
    }
}

impl Drop for GlobalState {
    fn drop(&mut self) {
        println!("GlobalState dropped");
    }
}
