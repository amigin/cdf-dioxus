pub enum RequestFail {
    InvalidUserNameOrPassword,
    TechnicalError,
}

impl RequestFail {
    pub fn as_str(&self) -> &str {
        match self {
            RequestFail::InvalidUserNameOrPassword => "Invalid user name or password",
            _ => "Technical error",
        }
    }

    pub fn from_i32(value: i32) -> Self {
        match value {
            1 => Self::InvalidUserNameOrPassword,
            _ => {
                println!("Invalid code error {}", value);
                Self::TechnicalError
            }
        }
    }
}
