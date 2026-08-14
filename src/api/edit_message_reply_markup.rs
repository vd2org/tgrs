use crate::*;
use bon::Builder;
use serde::Serialize;

#[derive(Builder, Serialize, Debug, Clone)]
#[builder(derive(Clone, Debug), on(String, into))]
/// A request to replace or remove a message's inline keyboard.
pub struct EditMessageReplyMarkup {
    /// The target chat identifier.
    pub chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The target message identifier.
    pub message_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The replacement keyboard, or `None` to remove it.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl TelegramRequest for EditMessageReplyMarkup {
    const METHOD: &'static str = "editMessageReplyMarkup";
}

impl TelegramRequestResponse<Message> for EditMessageReplyMarkup {}
