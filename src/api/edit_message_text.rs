use crate::*;
use bon::Builder;
use serde::Serialize;

#[derive(Builder, Serialize, Debug, Clone)]
#[builder(derive(Clone, Debug), on(String, into))]
/// A request to edit a text message.
pub struct EditMessageText {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The target chat identifier.
    pub chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The target message identifier.
    pub message_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The replacement text.
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// How Telegram should parse entities in the text.
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Link-preview settings for the replacement text.
    pub link_preview_options: Option<LinkPreviewOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Replacement reply markup.
    pub reply_markup: Option<ReplyMarkup>,
}

impl TelegramRequest for EditMessageText {
    const METHOD: &'static str = "editMessageText";
}

impl TelegramRequestResponse<Message> for EditMessageText {}
