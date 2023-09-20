use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use crate::error::AppError;

pub struct Jwt;

impl Jwt {
    pub fn encode<T: Serialize>(payload: &T) -> Result<String, AppError> {
        let secrete = std::env::var("ENC_KEY")?;
        let token = encode(
            &Header::default(),
            payload,
            &EncodingKey::from_secret(secrete.as_ref()),
        );

        match token {
            Ok(token) => Ok(token),
            Err(e) => Err(AppError::Message(format!("JWT encoding failed: {}", e))),
        }
    }

    pub fn decode<T: DeserializeOwned>(token: &str) -> Result<T, AppError> {
        let secrete = std::env::var("ENC_KEY")?;
        let token_payload = decode::<T>(
            token,
            &DecodingKey::from_secret(secrete.as_ref()),
            &Validation::default(),
        );
        match token_payload {
            Ok(token_payload) => Ok(token_payload.claims),
            Err(_) => Err(AppError::Message("Invalid Token".to_string())),
        }
    }
}

#[cfg(test)]
mod test {
    use chrono::{Duration, Utc};
    use serde::{Deserialize, Serialize};

    use crate::encryption::Jwt;
    use crate::load_env;

    #[derive(Serialize, Deserialize, Debug)]
    struct User {
        code: String,
        exp: i64,
    }

    /// if expiry date has passed, it should fail to decode the token.
    #[test]
    fn should_fail() {
        let user = User {
            code: "SAM".into(),
            exp: Utc::now().timestamp() - 1000,
        };
        load_env(None);
        let encoded = Jwt::encode(&user);
        assert!(encoded.is_ok());
        let decoded = Jwt::decode::<User>(&encoded.unwrap());
        println!("{:?}", decoded);
        assert!(decoded.is_err());
    }

    #[test]
    fn should_pass() {
        load_env(None);
        let user = User {
            code: "SAM".into(),
            exp: (Utc::now() + Duration::minutes(10)).timestamp(),
        };
        let encoded = Jwt::encode(&user);
        assert!(encoded.is_ok());
        let decoded = Jwt::decode::<User>(&encoded.unwrap());
        assert_eq!(decoded.unwrap().code, "SAM".to_string());
    }
}
