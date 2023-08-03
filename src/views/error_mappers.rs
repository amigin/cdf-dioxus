use dioxus_toast::{ToastInfo, ToastManager};
use fermi::UseAtomRef;

use crate::{grpc_client::TraderCredentialsRequestFail, lang::LANG};

impl TraderCredentialsRequestFail {
    pub fn throw_toast(&self, header: &str, toast: &UseAtomRef<ToastManager>) {
        let text = match self {
            TraderCredentialsRequestFail::TechnicalError => {
                LANG.toast_errors.technical_error.as_str()
            }
            TraderCredentialsRequestFail::TraderExists => {
                LANG.toast_errors.user_already_exists.as_str()
            }
            TraderCredentialsRequestFail::InvalidUsernameOrPassword => {
                LANG.toast_errors.user_already_exists.as_str()
            }
            TraderCredentialsRequestFail::TraderNoFound => {
                LANG.toast_errors.user_not_found.as_str()
            }
            TraderCredentialsRequestFail::PasswordIsWrong => {
                LANG.toast_errors.password_is_wrong.as_str()
            }
        };
        toast.write().popup(ToastInfo::error(text, header));

        /*
                match result {
            RegisterClientResult::Ok(trader_id) => {
                global_state.write().set_loading(trader_id);
            }
            RegisterClientResult::UserAlreadyExists => {
                toast.write().popup(ToastInfo::error(
                    &LANG.toast_errors.user_already_exists,
                    &LANG.toast_errors.registration_fail,
                ));
            }
            RegisterClientResult::TechnicalError => {
                toast.write().popup(ToastInfo::error(
                    &LANG.toast_errors.technical_error,
                    &LANG.toast_errors.registration_fail,
                ));
            }
        }
         */
    }
}
