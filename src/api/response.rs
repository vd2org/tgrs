use crate::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct OkResponse {
    pub result: TelegramResult,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ErrResponse {
    pub error_code: Option<i32>,
    pub description: Option<String>,
    // pub parameters: Option<ResponseParameters>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Response {
    Ok(OkResponse),
    Err(ErrResponse),
}
