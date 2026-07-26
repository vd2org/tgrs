use crate::*;
use serde::Serialize;
use std::fmt::Debug;
use std::future::Future;

pub trait TelegramRequest: Serialize + Send + Clone + Debug {
    const METHOD: &'static str;
}

pub trait TelegramRequestResponse<R: TelegramResponse>: Serialize + Debug + Send + Clone {
    #[inline]
    fn send(&self, telegram: &Telegram) -> impl Future<Output = Result<R, TelegramError>>
    where
        Self: TelegramRequest,
    {
        telegram.send(self)
    }
}

pub trait TelegramResponse: TryFrom<TelegramResult, Error = TelegramError> {}
