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
        I: IntoIterator<Item = InlineKeyboardButton>,
    {
        Self::builder().inline_keyboard(vec![buttons.into_iter().collect()]).build()
    }

    pub fn column<I>(buttons: I) -> Self
    where
        I: IntoIterator<Item = InlineKeyboardButton>,
    {
        Self::builder().inline_keyboard(buttons.into_iter().map(|b| vec![b]).collect()).build()
    }

    pub fn rows<I>(buttons: I, buttons_per_row: usize) -> Self
    where
        I: IntoIterator<Item = InlineKeyboardButton>,
    {
        assert!(buttons_per_row > 0);

        let mut rows = Vec::new();
        for button in buttons {
            if rows.last().is_none_or(|row: &Vec<InlineKeyboardButton>| row.len() == buttons_per_row) {
                rows.push(Vec::with_capacity(buttons_per_row));
            }
            rows.last_mut().unwrap().push(button);
        }

        Self::builder().inline_keyboard(rows).build()
    }
}

impl From<Vec<Vec<InlineKeyboardButton>>> for InlineKeyboardMarkup {
    fn from(v: Vec<Vec<InlineKeyboardButton>>) -> Self {
        InlineKeyboardMarkup { inline_keyboard: v }
    }
}

impl From<InlineKeyboardMarkup> for ReplyMarkup {
    fn from(v: InlineKeyboardMarkup) -> Self {
        ReplyMarkup::InlineKeyboardMarkup(v)
    }
}
