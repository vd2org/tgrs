use crate::*;
use bon::Builder;
use serde::Serialize;

#[derive(Builder, Serialize, Debug, Clone)]
#[builder(derive(Clone, Debug), on(String, into))]
pub struct GetMe {}

impl TelegramRequest for GetMe {
    const METHOD: &'static str = "getMe";
}

impl TelegramRequestResponse<User> for GetMe {}

pub const GET_ME: GetMe = GetMe {};
