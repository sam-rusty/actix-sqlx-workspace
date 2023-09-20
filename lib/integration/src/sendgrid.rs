use std::fs::read_to_string;

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use reqwest::{Client, StatusCode};
use serde::Serialize;
use serde_json::json;

use services::error::AppError;
use AppError::Message;

const SENDGRID_API_URL: &str = "https://api.sendgrid.com/v3/mail/send";

pub struct Recipient {
    email: String,
    name: String,
}

impl Recipient {
    pub fn new(email: String, first_name: String, last_name: String) -> Self {
        Self {
            email,
            name: format!("{first_name} {last_name}"),
        }
    }
}

pub type Sender = Recipient;

#[derive(Serialize)]
struct Attachment {
    content: String,
    filename: String,
    #[serde(rename = "type")]
    type_: String,
    disposition: String,
}

pub struct Email<'a> {
    recipient: Recipient,
    subject: &'a str,
    body: String,
    from: Option<Recipient>,
    cc: Option<Recipient>,
    attachments: Vec<Attachment>,
}

impl<'a> Email<'a> {
    pub fn new(recipient: Recipient, subject: &'a str, body: String) -> Self {
        Self {
            recipient,
            subject,
            body,
            from: None,
            cc: None,
            attachments: vec![],
        }
    }

    pub fn from(mut self, from: Sender) -> Self {
        self.from = Some(from);
        self
    }

    pub fn cc(mut self, cc: Recipient) -> Self {
        self.cc = Some(cc);
        self
    }

    pub fn with_attachment(
        mut self,
        filename: String,
        mime_type: String,
        file_path: Option<String>,
        content: Option<String>,
    ) -> Result<Self, AppError> {
        // read file content
        let content = match content {
            None => {
                if let Some(path) = file_path {
                    read_to_string(path).map_err(|_| Message("File not found".to_string()))?
                } else {
                    return Err(Message("File Path must be set".to_string()));
                }
            }
            Some(content) => content,
        };
        self.attachments.push(Attachment {
            content,
            filename,
            type_: mime_type,
            disposition: "attachment".to_string(),
        });
        Ok(self)
    }

    pub async fn send(self) -> Result<(), AppError> {
        if cfg!(test) {
            return Ok(());
        }
        let api_key = std::env::var("SENDGRID_API_KEY")?;
        let from = match self.from {
            None => Recipient::new(
                "support@hgicrusade.com".to_string(),
                "Hegemon Group".to_string(),
                "International".to_string(),
            ),
            Some(f) => f,
        };
        let client = Client::new();
        let mut header = HeaderMap::new();
        header.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", api_key)).map_err(|_| {
                AppError::Response(
                    "Invalid header value".to_string(),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            })?,
        );
        header.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let mut cc = json!({});
        if let Some(cc_info) = self.cc {
            cc = json!({
                "to": [{ "email": cc_info.email, "name": cc_info.name }]
            })
        }
        let json_payload = json!({
            "personalizations": [
                {
                    "to": [{ "email": self.recipient.email, "name": self.recipient.name }]
                },
                ..cc
            ],
            "from": {
                "name": from.name,
                "email": from.email
            },
            "subject": self.subject,
            "attachments": self.attachments,
            "content": [
                {
                    "type": "text/html",
                    "value": self.body
                }
            ]
        });

        let response = client
            .post(SENDGRID_API_URL)
            .headers(header)
            .json(&json_payload)
            .send()
            .await
            .map_err(|e| AppError::Response(e.to_string(), StatusCode::BAD_REQUEST))?;

        if response.status().is_success() {
            Ok(())
        } else {
            let message_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(AppError::Response(message_text, StatusCode::BAD_REQUEST))
        }
    }
}
