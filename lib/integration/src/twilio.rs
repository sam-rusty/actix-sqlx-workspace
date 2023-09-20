use std::collections::HashMap;

use reqwest::header::{HeaderMap, AUTHORIZATION};
use reqwest::{Client, StatusCode};

use services::error::AppError;

const TWILIO_API_URL: &str = "https://api.twilio.com/2010-04-01/Accounts/";

pub struct Sms;

impl Sms {
    pub async fn send(to: &str, body: &str) -> Result<(), AppError> {
        let account_sid = std::env::var("TWILIO_ACCOUNT_SID")?;
        let auth_token = std::env::var("TWILIO_AUTH_TOKEN")?;
        let from = std::env::var("TWILIO_FROM")?;

        let url = format!("{}{}/Messages.json", TWILIO_API_URL, account_sid);

        let client = Client::new();
        let mut request_body = HashMap::new();
        request_body.insert("To", to);
        request_body.insert("From", &from);
        request_body.insert("Body", body);

        let mut header = HeaderMap::new();

        let authorization_str = format!("Basic {}:{}", account_sid, auth_token);
        let header_value = authorization_str
            .parse()
            .map_err(|_| AppError::Message("Invalid header value".to_string()))?;
        header.insert(AUTHORIZATION, header_value);
        let response = client
            .post(&url)
            .form(&request_body)
            .headers(header)
            .send()
            .await
            .map_err(|e| AppError::Response(e.to_string(), StatusCode::BAD_REQUEST))?;
        if response.status().is_success() {
            Ok(())
        } else {
            let message_text = response.text().await.unwrap();
            Err(AppError::Response(message_text, StatusCode::BAD_REQUEST))
        }
    }
}
