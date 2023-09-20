use chrono::NaiveDate;
use http::StatusCode;
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use validator::Validate as ActixValidator;

use services::db::DBConnection;
use services::error::AppError;
use services::users::State;
use services::Country;
use AppError::Response;

#[derive(Serialize, ActixValidator, Deserialize, Clone)]
pub struct CreditCardInfo {
    pub token: String,
    pub last4: i32,
}

impl CreditCardInfo {
    pub fn validate(&self) -> Result<(), AppError> {
        if !self.token.starts_with("tok_") || self.last4.to_string().len() != 4 {
            return Err(Response(
                "Invalid Payment Information".to_string(),
                StatusCode::BAD_REQUEST,
            ));
        }
        Ok(())
    }
}

#[derive(Serialize, ActixValidator, Deserialize, Clone)]
pub struct RegistrationForm {
    pub first_name: String,
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(phone)]
    pub phone: String,
    pub user_name: String,
    pub state: State,
    pub country: Country,
    pub password: String,
    pub card: Option<CreditCardInfo>,
}

#[allow(unused)]
impl RegistrationForm {
    /// register new user after validating unique fields, check if coupon was applied,
    /// charge payment through stripe, Add to queue for RV points and hierarchy snapshot records.
    pub async fn register(&self, db: &DBConnection) -> Result<(), AppError> {
        if let Some(card) = &self.card {
            card.validate()?;
        }
        // do magic!
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
