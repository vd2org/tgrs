use crate::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
/// An incoming query produced by an inline keyboard button.
pub struct CallbackQuery {
    /// The unique callback query identifier.
    pub id: String,
    /// The user who triggered the callback.
    pub from: User,
    /// The callback payload assigned to the button.
    pub data: String,
    /// The message containing the button, when available.
    pub message: Option<Message>,
}
