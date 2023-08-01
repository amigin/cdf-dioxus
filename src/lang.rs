lazy_static::lazy_static! {
    pub static ref LANG: Language = Language {
        login: "Login".to_string(),
        password: "Password".to_string(),
        forgot_password: "Forgot Password?".to_string(),
        sign_up: "Sign Up".to_string(),
        login_failed: "Login failed".to_string(),
        total: "Total".to_string(),
        pnl_calculator: PnlCalculator {
            pnl_calculator_button: "Pnl Calculator".to_string(),
        },
    };
}

pub struct Language {
    pub login: String,
    pub password: String,
    pub forgot_password: String,
    pub sign_up: String,
    pub login_failed: String,
    pub total: String, // At the header of the account balance widget
    pub pnl_calculator: PnlCalculator,
}

pub struct PnlCalculator {
    pub pnl_calculator_button: String,
}
