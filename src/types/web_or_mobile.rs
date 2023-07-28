pub enum WebOrMobile {
    Web,
    Mobile,
}

impl WebOrMobile {
    pub fn is_web(&self) -> bool {
        match self {
            Self::Web => true,
            Self::Mobile => false,
        }
    }

    pub fn is_mobile(&self) -> bool {
        match self {
            Self::Web => false,
            Self::Mobile => true,
        }
    }
    pub fn to_i32(&self) -> i32 {
        match self {
            Self::Web => 0,
            Self::Mobile => 1,
        }
    }

    pub fn from_i32(src: i32) -> Self {
        match src {
            0 => Self::Web,
            1 => Self::Mobile,
            _ => {
                panic!("Invalid value for WebOrMobile: {}", src);
            }
        }
    }
}
