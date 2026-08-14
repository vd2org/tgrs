use crate::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
/// The subset of a Telegram message currently modeled by the crate.
pub struct Message {
    /// The unique message identifier within the chat.
    pub message_id: u64,
    /// The chat containing the message.
    pub chat: Chat,
}
