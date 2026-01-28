use email_address::EmailAddress;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize)]
pub struct ValidEmail(String);

impl ValidEmail {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for ValidEmail {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if EmailAddress::is_valid(&s) {
            Ok(ValidEmail(s))
        } else {
            Err(serde::de::Error::custom("Invalid email format"))
        }
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: ValidEmail,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct TryLuckResponse {
    pub win: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
