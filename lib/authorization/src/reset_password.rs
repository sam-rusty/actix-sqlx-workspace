use bcrypt::hash;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use validator::Validate as ActixValidator;

use services::db::DBConnection;
use services::error::AppError;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResetPasswordPayload {
    pub id: i32,
    pub user_name: String,
}

#[derive(Serialize, ActixValidator, Deserialize)]
pub struct ResetPassword {
    pub token: String,
    #[validate(length(
        min = 8,
        max = 64,
        message = "Password must be between 8 and 64 characters"
    ))]
    pub new_password: String,
    #[validate(must_match(other = "new_password", message = "Passwords do not match"))]
    pub re_type_password: String,
}

impl ResetPassword {
    pub async fn attempt(&self, db: &DBConnection) -> Result<(), AppError> {
        let payload = services::encryption::Jwt::decode::<ResetPasswordPayload>(&self.token)?;
        let password =
            hash(&self.new_password, 12).map_err(|e| AppError::Message(e.to_string()))?;
        let rows_affected = sqlx::query!(
            "UPDATE users SET password = $1 WHERE user_name = $2",
            password,
            payload.user_name
        )
        .execute(db)
        .await?
        .rows_affected();
        if rows_affected == 1 {
            return Ok(());
        }
        Err(AppError::Response(
            "Invalid Token".into(),
            StatusCode::BAD_REQUEST,
        ))
    }
}

#[cfg(test)]
mod tests {}
