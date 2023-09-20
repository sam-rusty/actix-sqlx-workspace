use bcrypt::verify;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use validator::Validate;

use services::db::DBConnection;
use services::error::AppError;
use services::middleware::UserClaim;
use AppError::Response;

#[derive(Serialize, Deserialize, Validate)]
pub struct Login {
    user_name: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    password: String,
}

impl Login {
    pub async fn login(&self, db: &DBConnection) -> Result<String, AppError> {
        // get account information
        let user = sqlx::query!(
            r#"
                SELECT
                    first_name, last_name, email, password, photo
                FROM users
                WHERE user_name = $1
            "#,
            self.user_name
        )
        .fetch_one(db)
        .await
        .map_err(|_| {
            Response(
                "Invalid username or password".to_string(),
                StatusCode::UNAUTHORIZED,
            )
        })?;

        // verify password hash
        if !verify(&self.password, &user.password).unwrap_or(false) {
            return Err(Response(
                "Invalid username or password".into(),
                StatusCode::UNAUTHORIZED,
            ));
        }

        let token = services::encryption::Jwt::encode(&UserClaim::new(
            user.first_name,
            user.last_name,
            user.email,
            user.photo,
        ))?;
        Ok(token)
    }
}

#[cfg(test)]
mod tests {}
