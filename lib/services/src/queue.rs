use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Serialize, Deserialize, Display, Debug)]
pub enum MessageType {
    Message,
    Email,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub type_: MessageType,
    pub payload: String,
}
