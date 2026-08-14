use crate::*;
use reqwest::Error as ReqwestError;
use std::error::Error;
use std::fmt;
use std::fmt::Display;

pub enum TelegramError {
    Api(ErrResponse),
    Type(TelegramResult),
    Network(ReqwestError),
}

impl fmt::Debug for TelegramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TelegramError::Api(response) => f.debug_tuple("Api").field(response).finish(),
            TelegramError::Type(result) => f.debug_tuple("Type").field(result).finish(),
            TelegramError::Network(error) => f
                .debug_struct("Network")
                .field("kind", &network_error_kind(error))
                .field("status", &error.status())
                .finish(),
        }
    }
}

impl Display for TelegramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TelegramError::Api(res) => match (&res.error_code, &res.description) {
                (Some(code), Some(description)) => {
                    write!(f, "Telegram API error {code}: {description}")
                }
                (Some(code), None) => write!(f, "Telegram API error {code}"),
                (None, Some(description)) => write!(f, "Telegram API error: {description}"),
                (None, None) => f.write_str("Telegram API returned an unspecified error"),
            },
            TelegramError::Type(res) => write!(f, "Unexpected result type: {:?}", res),
            TelegramError::Network(err) => {
                write!(f, "Telegram request failed: {}", network_error_kind(err))?;
                if let Some(status) = err.status() {
                    write!(f, " ({status})")?;
                }
                Ok(())
            }
        }
    }
}

impl From<ReqwestError> for TelegramError {
    fn from(e: ReqwestError) -> Self {
        TelegramError::Network(e.without_url())
    }
}

impl Error for TelegramError {}

fn network_error_kind(error: &ReqwestError) -> &'static str {
    if error.is_timeout() {
        "request timed out"
    } else if error.is_connect() {
        "connection failed"
    } else if error.is_decode() {
        "response could not be decoded"
    } else if error.is_body() {
        "response body failed"
    } else if error.is_status() {
        "HTTP status error"
    } else if error.is_request() {
        "request could not be sent"
    } else {
        "unknown request error"
    }
}
