use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub enum ParseMode {
    #[serde(rename = "HTML")]
    HTML,
    #[serde(rename = "MarkdownV2")]
    MarkdownV2,
    #[serde(rename = "Markdown")]
    Markdown,
}
