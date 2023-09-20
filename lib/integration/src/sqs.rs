use std::io::ErrorKind;

use aws_sdk_sqs::Client;
use serde_json::json;

use services::queue::Message;

pub struct Sqs {
    client: Client,
    queue_url: String,
}

impl Sqs {
    // create new sqs client
    pub async fn new(queue_url: String) -> Self {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);
        Self { client, queue_url }
    }

    pub async fn add(&self, message: Message) -> Result<(), std::io::Error> {
        let message = json!(message).to_string();
        // if cfg!(test) {
        //     return Ok(());
        // }
        println!("message {message}");
        let response = self
            .client
            .send_message()
            .set_queue_url(Some(self.queue_url.clone()))
            .message_body(message)
            .send()
            .await;
        match response {
            Ok(e) => {
                println!("{:?}", e);
                Ok(())
            }
            Err(e) => Err(std::io::Error::new(ErrorKind::Other, e.to_string())),
        }
    }
}
