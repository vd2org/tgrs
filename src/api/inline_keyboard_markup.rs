use crate::*;
use bon::Builder;
use serde::Serialize;

#[derive(Builder, Serialize, Debug, Clone)]
#[builder(derive(Clone, Debug), on(String, into))]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

impl InlineKeyboardMarkup {
    pub fn row<I>(buttons: I) -> Self
    where
        I: IntoIterator<Item=InlineKeyboardButton>,
    {
        Self::builder()
            .inline_keyboard(vec![buttons.into_iter().collect()])
            .build()
    }

    pub fn column<I>(buttons: I) -> Self
    where
        I: IntoIterator<Item=InlineKeyboardButton>,
    {
        Self::builder()
            .inline_keyboard(buttons.into_iter().map(|b| vec![b]).collect())
            .build()
    }
}

impl From<InlineKeyboardMarkup> for ReplyMarkup {
    fn from(v: InlineKeyboardMarkup) -> Self {
        ReplyMarkup::InlineKeyboardMarkup(v)
    }
}
