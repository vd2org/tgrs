use crate::*;
use bon::Builder;
use serde::Serialize;

#[derive(Builder, Serialize, Debug, Clone)]
#[builder(derive(Clone, Debug), on(String, into))]
/// A request to delete multiple messages from a chat.
pub struct DeleteMessages {
    /// The target chat identifier.
    pub chat_id: i64,
    /// The identifiers of messages to delete.
    pub message_ids: Vec<u64>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub text: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub show_alert: Option<bool>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub url: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub cache_time: Option<u32>,
}

impl TelegramRequest for DeleteMessages {
    const METHOD: &'static str = "deleteMessages";
}

impl TelegramRequestResponse<bool> for DeleteMessages {}
