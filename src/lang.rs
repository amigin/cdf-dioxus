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

        sign_up_form : SignUpForm{
            email: "Email".to_string(),
            password: "Password".to_string(),
            password_again: "Password again".to_string(),
        },

        toast_errors: ToastErrors {
            authentication_fail: "Authentication failed".to_string(),
            registration_fail: "Registration failed".to_string(),

            user_already_exists: "User already exists".to_string(),
            passwords_do_not_match: "Passwords do not match".to_string(),
            technical_error: "Technical error".to_string(),
            invalid_user_name_or_password: "Invalid user name or password".to_string(),
            user_not_found: "User not found".to_string(),
            password_is_wrong: "Password is wrong".to_string(),
        },

        reset_password_form: ResetPasswordForm{
            reset_password_title: "Reset Password".to_string(),
            to_begin_changing_password_under_title: "To begin changing your password, please enter your e-mail".to_string(),
            email_input: "Email".to_string(),
            send_recovery_code: "Send recovery code".to_string(),
            back_to_login: "Back to login".to_string()
        }


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
    pub sign_up_form: SignUpForm,
    pub toast_errors: ToastErrors,
    pub reset_password_form: ResetPasswordForm,
}

pub struct PnlCalculator {
    pub pnl_calculator_button: String,
}

pub struct SignUpForm {
    pub email: String,
    pub password: String,
    pub password_again: String,
}

pub struct ResetPasswordForm {
    pub reset_password_title: String,
    pub to_begin_changing_password_under_title: String,
    pub email_input: String,
    pub send_recovery_code: String,
    pub back_to_login: String,
}

pub struct ToastErrors {
    //Headers
    pub authentication_fail: String,
    pub registration_fail: String,

    //Messages
    pub technical_error: String,
    pub user_already_exists: String,
    pub user_not_found: String,
    pub invalid_user_name_or_password: String,
    pub passwords_do_not_match: String,
    pub password_is_wrong: String,
}
