use actix_web::dev::ServiceRequest;
use actix_web::HttpMessage;
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};

const NO_LOGIN_REQUIRED: [&str; 2] = ["/", "/authorization"];

#[derive(Serialize, Deserialize)]
pub struct UserClaim {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub photo: Option<String>,
    pub exp: i64,
}

impl UserClaim {
    pub fn new(
        first_name: String,
        last_name: String,
        email: String,
        photo: Option<String>,
    ) -> Self {
        Self {
            first_name,
            last_name,
            email,
            photo,
            exp: (Utc::now() + Duration::hours(24)).timestamp(),
        }
    }
}

pub struct Middleware;

impl Middleware {
    pub fn check_login(req: &ServiceRequest) -> bool {
        let path = req.path();
        if NO_LOGIN_REQUIRED.iter().any(|e| e.starts_with(path)) {
            return true;
        }
        let headers = req.headers();
        if let Some(token) = headers.get("Authorization") {
            let token = token.to_str().unwrap();
            if let Ok(payload) = crate::encryption::Jwt::decode::<UserClaim>(token) {
                req.extensions_mut().insert(payload);
                return true;
            }
        }
        false
    }
}
