use crate::*;
use log::trace;
use reqwest::{Client as RequestClient, Error as RequestError};
use std::fmt::Debug;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Telegram {
    client: RequestClient,
    token: String,
}

impl Telegram {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            client: RequestClient::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap(),
            token: token.into(),
        }
    }

    pub async fn call_raw<T>(self: &Self, payload: &T) -> Result<Response, RequestError>
    where
        T: TelegramRequest,
    {
        trace!("Sending request {} {:?}", T::METHOD, payload);

        let url = format!("https://api.telegram.org/bot{}/{}", self.token, T::METHOD);

        let http_response = self.client.post(&url).json(&payload).send().await?;

        let api_response: Response = http_response.json().await?;

        trace!("Received response for {}: {:?}", T::METHOD, api_response);

        Ok(api_response)
    }

    pub async fn send<R, T>(self: &Self, payload: &T) -> Result<R, TelegramError>
    where
        T: TelegramRequest + TelegramRequestResponse<R>,
        R: TelegramResponse,
    {
        match self.call_raw(payload).await {
            Ok(Response::Ok(response)) => R::try_from(response.result),
            Ok(Response::Err(response)) => Err(TelegramError::Api(response)),
            Err(e) => Err(TelegramError::Network(e)),
        }
    }
}
