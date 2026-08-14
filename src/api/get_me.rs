use crate::*;
use bon::Builder;
use serde::Serialize;

#[derive(Builder, Serialize, Debug, Clone)]
#[builder(derive(Clone, Debug), on(String, into))]
/// A request for information about the authenticated bot.
pub struct GetMe {}

impl TelegramRequest for GetMe {
    const METHOD: &'static str = "getMe";
}

impl TelegramRequestResponse<User> for GetMe {}

/// A reusable `getMe` request value.
pub const GET_ME: GetMe = GetMe {};
