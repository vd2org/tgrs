use crate::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Message {
    pub message_id: u64,
    pub chat: Chat,
}
