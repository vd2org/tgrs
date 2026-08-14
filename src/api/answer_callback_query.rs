use crate::*;
use bon::Builder;
use serde::Serialize;

#[derive(Builder, Serialize, Debug, Clone)]
#[builder(derive(Clone, Debug), on(String, into))]
/// A request that acknowledges an inline keyboard callback query.
pub struct AnswerCallbackQuery {
    /// The identifier of the callback query to answer.
    pub callback_query_id: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub text: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub show_alert: Option<bool>,
    // #[serde(skip_serializing_if = "Option::is_none")]x
    // pub url: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub cache_time: Option<u32>,
}

impl AnswerCallbackQuery {
    /// Creates an acknowledgement without a message or alert.
    pub fn simple(id: impl Into<String>) -> Self {
        Self {
            callback_query_id: id.into(),
        }
    }
}

impl TelegramRequest for AnswerCallbackQuery {
    const METHOD: &'static str = "answerCallbackQuery";
}

impl TelegramRequestResponse<bool> for AnswerCallbackQuery {}
