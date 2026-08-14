#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

/// Telegram Bot API request and response types.
pub mod api;
/// Errors returned while sending or decoding Telegram requests.
pub mod error;
/// The HTTP client used to call Telegram methods.
pub mod telegram;
/// Traits connecting requests to their possible response types.
pub mod types;

pub use api::*;
pub use error::*;
pub use telegram::*;
pub use types::*;
