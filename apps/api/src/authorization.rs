use actix_web::{
    web::{self, post},
    Responder,
};
use actix_web_validator::Json;
use serde_json::json;

use authorization::forget_password::ForgetPassword;
use authorization::login;
use authorization::register::RegistrationForm;
use authorization::reset_password::ResetPassword;
use services::db::DBConnection;
use services::error::AppError;
use services::response::Response;

pub async fn login_handler(
    db: web::Data<DBConnection>,
    form: Json<login::Login>,
) -> Result<impl Responder, AppError> {
    let token = form.login(&db).await?;
    let response = json!({ "token": token });
    Ok(web::Json(response))
}

pub async fn register_handler(
    db: web::Data<DBConnection>,
    form: Json<RegistrationForm>,
) -> Result<impl Responder, AppError> {
    form.register(&db).await?;
    Response::ok()
}

pub async fn forget_password_handler(
    db: web::Data<DBConnection>,
    form: Json<ForgetPassword>,
) -> Result<impl Responder, AppError> {
    form.attempt(&db).await?;
    Response::ok()
}

pub async fn reset_password_handler(
    db: web::Data<DBConnection>,
    form: Json<ResetPassword>,
) -> Result<impl Responder, AppError> {
    form.attempt(&db).await?;
    Response::ok()
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // scope will add prefix to all the routes in this module
        web::scope("/authorization")
            .route("/login", post().to(login_handler))
            .route("/register", post().to(register_handler))
            .route("/forget-password", post().to(forget_password_handler))
            .route("/reset-password", post().to(reset_password_handler)),
    );
}
