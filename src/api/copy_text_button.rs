use crate::*;
use bon::Builder;
use serde::Serialize;

#[derive(Builder, Serialize, Debug, Clone)]
#[builder(derive(Clone, Debug), on(String, into))]
pub struct CopyTextButton {
    pub text: String,
}

impl<T> From<T> for CopyTextButton
where
    T: Into<String>,
{
    fn from(s: T) -> Self {
        CopyTextButton { text: s.into() }
    }
}
