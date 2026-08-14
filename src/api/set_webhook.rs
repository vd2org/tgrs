use crate::*;
use bon::Builder;
use serde::Serialize;
use std::fmt::{self, Debug};

#[derive(Builder, Serialize, Clone)]
#[builder(derive(Clone), on(String, into), on(Into<Vec<String>>, into))]
pub struct SetWebhook {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_token: Option<String>,
}

impl Debug for SetWebhook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SetWebhook")
            .field("url", &self.url)
            .field("max_connections", &self.max_connections)
            .field("allowed_updates", &self.allowed_updates)
            .field("drop_pending_updates", &self.drop_pending_updates)
            .field(
                "secret_token",
                &self.secret_token.as_ref().map(|_| "[REDACTED]"),
            )
            .finish()
    }
}

impl TelegramRequest for SetWebhook {
    const METHOD: &'static str = "setWebhook";
}

impl TelegramRequestResponse<bool> for SetWebhook {}
