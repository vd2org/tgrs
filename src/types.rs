use crate::*;
use serde::Serialize;
use std::fmt::Debug;
use std::future::Future;

pub trait TelegramRequest: Serialize + Send + Clone + Debug {
    const METHOD: &'static str;
}

pub trait TelegramRequestResponse<R: TelegramResponse>: TelegramRequest {
    #[inline]
    fn send(&self, telegram: &Telegram) -> impl Future<Output = Result<R, TelegramError>> {
        telegram.send(self)
    }
}

pub trait TelegramResponse: TryFrom<TelegramResult, Error = TelegramError> {}
