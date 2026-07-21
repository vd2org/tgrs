use crate::*;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
}
