use crate::*;
use serde::Serialize;
use std::fmt::Debug;
use std::future::Future;

/// A serializable Telegram Bot API method request.
pub trait TelegramRequest: Serialize + Send + Clone + Debug {
    /// The Telegram Bot API method name.
    const METHOD: &'static str;
}

/// Marks `R` as a response type supported by this request.
///
/// A request may implement this trait for more than one response type when a
/// Telegram method's result depends on context known by the caller.
pub trait TelegramRequestResponse<R: TelegramResponse>: TelegramRequest {
    /// Sends this request and converts the result to `R`.
    #[inline]
    fn send(&self, telegram: &Telegram) -> impl Future<Output = Result<R, TelegramError>> {
        telegram.send(self)
    }
}

/// A type that can be extracted from a supported Telegram result.
pub trait TelegramResponse: TryFrom<TelegramResult, Error = TelegramError> {}
