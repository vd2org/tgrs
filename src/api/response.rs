use crate::*;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug, Clone)]
/// A successful Telegram API response body.
pub struct OkResponse {
    /// The decoded method result.
    pub result: TelegramResult,
}

#[derive(Deserialize, Debug, Clone)]
/// An unsuccessful Telegram API response body.
pub struct ErrResponse {
    /// Telegram's numeric error code, when supplied.
    pub error_code: Option<i32>,
    /// A human-readable explanation of the error.
    pub description: Option<String>,
    // pub parameters: Option<ResponseParameters>,
}

#[derive(Debug, Clone)]
/// A Telegram API response, discriminated by its `ok` field.
pub enum Response {
    /// A response with `ok: true`.
    Ok(OkResponse),
    /// A response with `ok: false`.
    Err(ErrResponse),
}

impl<'de> Deserialize<'de> for Response {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Envelope {
            ok: bool,
            result: Option<TelegramResult>,
            error_code: Option<i32>,
            description: Option<String>,
        }

        let envelope = Envelope::deserialize(deserializer)?;

        if envelope.ok {
            let result = envelope
                .result
                .ok_or_else(|| D::Error::missing_field("result"))?;
            Ok(Response::Ok(OkResponse { result }))
        } else {
            Ok(Response::Err(ErrResponse {
                error_code: envelope.error_code,
                description: envelope.description,
            }))
        }
    }
}
