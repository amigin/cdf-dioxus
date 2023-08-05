use encryption::aes::{AesEncryptedData, AesKey};
use rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionToken {
    #[prost(string, tag = "1")]
    pub trader_id: String,
    #[prost(int64, tag = "2")]
    pub expires: i64,
}

impl SessionToken {
    pub fn new(trader_id: String) -> Self {
        let mut expires = DateTimeAsMicroseconds::now();
        expires.add_days(1);
        Self {
            trader_id,
            expires: expires.unix_microseconds,
        }
    }

    pub fn to_string(&self, aes_key: &AesKey) -> String {
        let mut payload = Vec::new();
        prost::Message::encode(self, &mut payload).unwrap();

        let result = aes_key.encrypt(payload.as_slice());

        result.as_base_64()
    }

    pub fn is_expired(&self, now: DateTimeAsMicroseconds) -> bool {
        let expires = DateTimeAsMicroseconds::new(self.expires);
        now.unix_microseconds > expires.unix_microseconds
    }

    pub fn from_string(token: &str, aes_key: &AesKey) -> Result<Self, String> {
        let encrypted = AesEncryptedData::from_base_64(token).map_err(|_| "Invalid token")?;

        let decrypted = aes_key.decrypt(&encrypted).map_err(|_| "Invalid token")?;

        let result = prost::Message::decode(decrypted.as_slice()).map_err(|_| "Invalid token")?;

        Ok(result)
    }
}
