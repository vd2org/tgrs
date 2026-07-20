use crate::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum TelegramResult {
    Bool(bool),
    Message(Message),
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
