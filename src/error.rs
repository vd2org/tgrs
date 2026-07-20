use crate::*;
use reqwest::Error as RequestError;
use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum TelegramError {
    Api(ErrResponse),
    Type(TelegramResult),
    Network(RequestError),
}

impl Display for TelegramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TelegramError::Api(res) => write!(f, "API error occurred with status: {:?}", res),
            TelegramError::Type(res) => write!(f, "Unexpected result type: {:?}", res),
            TelegramError::Network(err) => write!(f, "Network connectivity issue: {}", err),
        }
    }
}

impl From<RequestError> for TelegramError {
    fn from(e: RequestError) -> Self {
        TelegramError::Network(e)
    }
}

impl Error for TelegramError {}
