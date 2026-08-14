use crate::*;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
/// Reply markup types currently supported by the crate.
pub enum ReplyMarkup {
    /// An inline keyboard attached to a message.
    InlineKeyboardMarkup(InlineKeyboardMarkup),
}
