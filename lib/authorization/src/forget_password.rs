use std::env::var;

use chrono::{Duration, Utc};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use validator::Validate as ActixValidator;

use integration::sendgrid::Recipient;
use services::db::DBConnection;
use services::error::AppError;

#[derive(Serialize, Deserialize, Debug)]
pub struct ForgotEmailPayload {
    pub user_name: String,
    pub exp: i64,
}

#[derive(Serialize, ActixValidator, Deserialize)]
pub struct ForgetPassword {
    #[validate(email)]
    pub email: String,
}

impl ForgetPassword {
    pub async fn attempt(&self, db: &DBConnection) -> Result<(), AppError> {
        let user = sqlx::query!(
            r#"SELECT first_name, last_name, user_name, email FROM users WHERE email = $1"#,
            self.email
        )
        .fetch_one(db)
        .await
        .map_err(|_| {
            AppError::Response(
                "User with this email does not exist.".into(),
                StatusCode::CONFLICT,
            )
        })?;
        let payload = ForgotEmailPayload {
            user_name: user.user_name,
            exp: (Utc::now() + Duration::minutes(10)).timestamp(),
        };
        let token = services::encryption::Jwt::encode(&payload)?;
        let url = var("SITE_URL").unwrap_or("http://domain.com/".into());
        let message = format!(
            r#"
                <p>Hi {} {},</p>
                <p>Please <a href='{url}reset-password/{token}'>click here</a> to to reset your password.</p>
            "#,
            user.first_name, user.last_name
        );
        integration::sendgrid::Email::new(
            Recipient::new(user.email, user.first_name, user.last_name),
            "Reset Password Link",
            message,
        )
        .send()
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
