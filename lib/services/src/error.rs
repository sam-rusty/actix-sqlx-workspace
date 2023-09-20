use std::error::Error;

use actix_web::{error::ResponseError, HttpResponse};
use actix_web_validator;
use actix_web_validator::JsonConfig;
use http::StatusCode;
use serde::Serialize;
use thiserror::Error;

use crate::db::DbErr;

pub type ErrorMessage = String;
pub type Cause = String;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum AppError {
    #[error("{0}")]
    Response(ErrorMessage, StatusCode),

    #[error("{0}")]
    ResponseWithCause(ErrorMessage, StatusCode, Cause),

    #[error("{0} Not Found")]
    NotFound(ErrorMessage),

    #[error("Env Var Error {0}")]
    EnvVarError(#[from] std::env::VarError),

    #[error("{0}")]
    MessageWithCause(ErrorMessage, Cause),

    #[error("{0}")]
    Message(ErrorMessage),

    #[error("{0}")]
    DbError(#[from] DbErr),

    #[error("{0}")]
    JsonParsingError(#[from] serde_json::Error),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::DbError(_)
            | Self::JsonParsingError(_)
            | Self::EnvVarError(_)
            | Self::Message(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::MessageWithCause(_, _) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Response(_, code) => code.to_owned(),
            Self::ResponseWithCause(_, code, _) => code.to_owned(),
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            error: self.to_string(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

pub fn actix_error_handler() -> JsonConfig {
    JsonConfig::default().limit(4096).error_handler(|err, _| {
        // Payload error: Json deserialize error:
        let err = err.source();
        let mut error_message = "".to_string();
        if let Some(error) = err {
            error_message = error
                .to_string()
                .replace("Json deserialize error: ", "")
                .split("at line ")
                .next()
                .unwrap_or("")
                .to_string()
        };
        AppError::Response(error_message, StatusCode::CONFLICT).into()
    })
}
