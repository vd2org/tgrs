use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
/// A mode used by Telegram to parse formatting entities in text.
pub enum ParseMode {
    #[serde(rename = "HTML")]
    /// HTML formatting.
    HTML,
    #[serde(rename = "MarkdownV2")]
    /// MarkdownV2 formatting.
    MarkdownV2,
    #[serde(rename = "Markdown")]
    /// Legacy Markdown formatting.
    Markdown,
}
