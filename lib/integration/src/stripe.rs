use std::collections::HashMap;
use std::convert::Into;
use std::string::ToString;

use reqwest::header::{HeaderMap, AUTHORIZATION};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use services::Country;
use strum_macros;
use strum_macros::AsRefStr;

use services::error::AppError;

const STRIPE_API_URL: &str = "https://api.stripe.com/v1";

#[derive(Deserialize, Serialize)]
pub struct Charge {
    id: String,
    status: String,
}

#[derive(Deserialize, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Deserialize, Serialize)]
struct StripeError {
    error: ErrorResponse,
}

pub struct Payment;

#[derive(AsRefStr)]
pub enum Currency {
    USD,
    CAD,
}

impl Payment {
    /// charge user for the amount specified in the currency of the country (USD or CAD)
    pub async fn charge(
        _first_name: &str,
        _last_name: &str,
        email: &str,
        amount: f32,
        country: &Country,
        source: &str,
    ) -> Result<String, AppError> {
        let mut data_map = HashMap::new();
        data_map.insert("receipt_email", email.to_string());
        data_map.insert("amount", (amount * 100.00).to_string());
        data_map.insert("source", source.to_string());

        // todo:: fix metadata issue, it was failing
        // metadata to store name of the user;
        // let mut meta_data = HashMap::new();
        // meta_data.insert("first_name", first_name.to_string());
        // meta_data.insert("last_name", last_name.to_string());
        // data_map.insert("metadata", serde_json::to_string(&meta_data)?);

        let body = Self::make_post_call("/charges", &mut data_map, country).await?;
        let response: Charge = serde_json::from_str(&body)?;
        Ok(response.id)
    }

    async fn make_post_call(
        endpoint: &str,
        data_map: &mut HashMap<&str, String>,
        country: &Country,
    ) -> Result<String, AppError> {
        let (secret_key, currency) = match country {
            Country::CA => (std::env::var("STRIPE_SECRET_KEY_CA")?, Currency::CAD),
            Country::US => (std::env::var("STRIPE_SECRET_KEY_US")?, Currency::USD),
        };
        data_map.insert("currency", currency.as_ref().to_string());
        let secret_key = secret_key
            .parse()
            .map_err(|_| AppError::Message("Invalid Stripe Key".into()))?;

        let client = Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, secret_key);
        let response = client
            .post(STRIPE_API_URL.clone().to_owned() + endpoint)
            .form(&data_map)
            .headers(headers)
            .send()
            .await
            .map_err(|e| AppError::Response(e.to_string(), StatusCode::BAD_REQUEST))?;
        let status = response.status();
        if !status.is_success() {
            let message_text = response.text().await.unwrap();
            let error_response: StripeError = serde_json::from_str(&message_text)?;
            Err(AppError::Response(
                error_response.error.message,
                StatusCode::BAD_REQUEST,
            ))
        } else {
            Ok(response.text().await.unwrap())
        }
    }
}
