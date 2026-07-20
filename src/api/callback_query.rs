use crate::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub data: String,
    pub message: Option<Message>,
}
