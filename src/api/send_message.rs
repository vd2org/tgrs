use crate::*;
use bon::Builder;
use serde::Serialize;

#[derive(Builder, Serialize, Debug, Clone)]
#[builder(derive(Clone, Debug), on(String, into))]
/// A request to send a text message.
pub struct SendMessage {
    /// The target chat identifier.
    pub chat_id: i64,
    /// The message text.
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Link-preview settings for the message.
    pub link_preview_options: Option<LinkPreviewOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// How Telegram should parse formatting entities in the text.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Reply markup attached to the message.
    pub reply_markup: Option<ReplyMarkup>,
}

impl TelegramRequest for SendMessage {
    const METHOD: &'static str = "sendMessage";
}

impl TelegramRequestResponse<Message> for SendMessage {}
