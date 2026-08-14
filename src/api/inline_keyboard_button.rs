use crate::*;
use bon::Builder;
use serde::Serialize;

#[derive(Builder, Serialize, Debug, Clone)]
#[builder(derive(Clone, Debug), on(String, into))]
/// A button displayed in an inline keyboard.
pub struct InlineKeyboardButton {
    /// The button label.
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Data sent back in a callback query when pressed.
    pub callback_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The visual style of the button.
    pub style: Option<ButtonStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Text copied to the clipboard when pressed.
    pub copy_text: Option<CopyTextButton>,
}
