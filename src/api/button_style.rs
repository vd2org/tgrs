use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
/// The visual style of an inline keyboard button.
pub enum ButtonStyle {
    /// A destructive or dangerous action.
    Danger,
    /// A successful or affirmative action.
    Success,
    /// A primary action.
    Primary,
}
