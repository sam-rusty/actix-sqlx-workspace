use actix_web::HttpResponse;
use http::StatusCode;
use serde::Serialize;
use serde_json::json;

use crate::error::AppError;

pub struct Response;

impl Response {
    pub fn inserted_id(id: i32) -> Result<(String, StatusCode), AppError> {
        let json = json!({
            "id": id
        });
        Ok((json.to_string(), StatusCode::CREATED))
    }

    pub fn result<T: Serialize>(result: T) -> Result<HttpResponse, AppError> {
        let json = json!({
            "result": result
        });
        Ok(HttpResponse::Ok().json(json))
    }

    pub fn ok<'a>() -> Result<&'a str, AppError> {
        Ok("Ok")
    }
}
