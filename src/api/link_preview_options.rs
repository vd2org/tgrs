use bon::Builder;
use serde::Serialize;

#[derive(Builder, Serialize, Debug, Clone)]
#[builder(derive(Clone, Debug), on(String, into))]
/// Options controlling link previews in a message.
pub struct LinkPreviewOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether link previews should be disabled.
    pub is_disabled: Option<bool>,
}
