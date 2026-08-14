use crate::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
/// A successful result type currently supported by the crate.
pub enum TelegramResult {
    /// A Boolean result.
    Bool(bool),
    /// A message result.
    Message(Message),
    /// A user or bot result.
    User(User),
}

impl TelegramResponse for bool {}

impl TryFrom<TelegramResult> for bool {
    type Error = TelegramError;

    fn try_from(value: TelegramResult) -> Result<Self, TelegramError> {
        match value {
            TelegramResult::Bool(inner) => Ok(inner),
            other => Err(TelegramError::Type(other)),
        }
    }
}

impl TelegramResponse for Message {}

impl TryFrom<TelegramResult> for Message {
    type Error = TelegramError;

    fn try_from(value: TelegramResult) -> Result<Self, TelegramError> {
        match value {
            TelegramResult::Message(inner) => Ok(inner),
            other => Err(TelegramError::Type(other)),
        }
    }
}

impl TelegramResponse for User {}

impl TryFrom<TelegramResult> for User {
    type Error = TelegramError;

    fn try_from(value: TelegramResult) -> Result<Self, TelegramError> {
        match value {
            TelegramResult::User(inner) => Ok(inner),
            other => Err(TelegramError::Type(other)),
        }
    }
}
