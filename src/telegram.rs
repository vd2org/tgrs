use crate::*;
use log::trace;
use reqwest::{Client as ReqwestClient, Error as ReqwestError};
use std::fmt::{self, Debug};
use std::time::Duration;

#[derive(Clone)]
/// A reusable client for sending Telegram Bot API requests.
///
/// Clones share the underlying HTTP connection pool. Debug output never
/// includes the bot token.
pub struct Telegram {
    client: ReqwestClient,
    token: String,
}

impl Debug for Telegram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Telegram")
            .field("client", &self.client)
            .field("token", &"[REDACTED]")
            .finish()
    }
}

impl Telegram {
    /// Creates a client using `token` and a ten-second request timeout.
    pub fn new(token: impl Into<String>) -> Result<Self, ReqwestError> {
        let client = ReqwestClient::builder()
            .timeout(Duration::from_secs(10))
            .build()?;
        Ok(Self {
            client,
            token: token.into(),
        })
    }

    /// Sends `payload` and returns the decoded Telegram response envelope.
    ///
    /// Unsupported successful result types are returned as decoding errors.
    pub async fn call_raw<T>(&self, payload: &T) -> Result<Response, ReqwestError>
    where
        T: TelegramRequest,
    {
        trace!("Sending request {} {:?}", T::METHOD, payload);

        let url = format!("https://api.telegram.org/bot{}/{}", self.token, T::METHOD);

        let http_response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(ReqwestError::without_url)?;

        let api_response: Response = http_response
            .json()
            .await
            .map_err(ReqwestError::without_url)?;

        trace!("Received response for {}: {:?}", T::METHOD, api_response);

        Ok(api_response)
    }

    /// Sends `payload` and converts its successful result to `R`.
    ///
    /// The caller selects `R`, which must be one of the response types
    /// supported by the request.
    pub async fn send<R, T>(&self, payload: &T) -> Result<R, TelegramError>
    where
        T: TelegramRequestResponse<R>,
        R: TelegramResponse,
    {
        match self.call_raw(payload).await {
            Ok(Response::Ok(response)) => R::try_from(response.result),
            Ok(Response::Err(response)) => Err(TelegramError::Api(response)),
            Err(e) => Err(e.into()),
        }
    }
}
