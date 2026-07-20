use crate::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Update {
    pub update_id: u64,
    pub message: Option<Message>,
    pub callback_query: Option<CallbackQuery>,
}
