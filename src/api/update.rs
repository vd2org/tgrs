use crate::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
/// An incoming update delivered by Telegram.
pub struct Update {
    /// The unique update identifier.
    pub update_id: u64,
    /// A newly received message, when present.
    pub message: Option<Message>,
    /// An inline keyboard callback, when present.
    pub callback_query: Option<CallbackQuery>,
}
