use crate::*;
use bon::Builder;
use serde::Serialize;

#[derive(Builder, Serialize, Debug, Clone)]
#[builder(derive(Clone, Debug), on(String, into))]
/// An inline keyboard arranged as rows of buttons.
pub struct InlineKeyboardMarkup {
    /// The keyboard rows, from top to bottom.
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

impl InlineKeyboardMarkup {
    /// Creates a keyboard containing one row.
    pub fn row<I>(buttons: I) -> Self
    where
        I: IntoIterator<Item = InlineKeyboardButton>,
    {
        Self::builder()
            .inline_keyboard(vec![buttons.into_iter().collect()])
            .build()
    }

    /// Creates a keyboard containing one button per row.
    pub fn column<I>(buttons: I) -> Self
    where
        I: IntoIterator<Item = InlineKeyboardButton>,
    {
        Self::builder()
            .inline_keyboard(buttons.into_iter().map(|b| vec![b]).collect())
            .build()
    }

    /// Groups buttons into rows containing at most `buttons_per_row` buttons.
    ///
    /// # Panics
    ///
    /// Panics if `buttons_per_row` is zero.
    pub fn rows<I>(buttons: I, buttons_per_row: usize) -> Self
    where
        I: IntoIterator<Item = InlineKeyboardButton>,
    {
        assert!(buttons_per_row > 0);

        let mut rows = Vec::new();
        for button in buttons {
            if rows
                .last()
                .is_none_or(|row: &Vec<InlineKeyboardButton>| row.len() == buttons_per_row)
            {
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
