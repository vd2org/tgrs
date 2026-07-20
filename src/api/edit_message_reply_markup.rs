use crate::*;
use bon::Builder;
use serde::Serialize;

#[derive(Builder, Serialize, Debug, Clone)]
#[builder(derive(Clone, Debug), on(String, into))]
pub struct EditMessageReplyMarkup {
    pub chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl TelegramRequest for EditMessageReplyMarkup {
    const METHOD: &'static str = "editMessageReplyMarkup";
}

impl TelegramRequestResponse<Message> for EditMessageReplyMarkup {}
