use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
/// The kind of a Telegram chat.
pub enum ChatType {
    /// A private conversation with a user.
    Private,
    /// A basic group.
    Group,
    /// A supergroup.
    Supergroup,
    /// A channel.
    Channel,
}

#[derive(Deserialize, Debug, Clone)]
/// A Telegram chat associated with a message.
pub struct Chat {
    /// The unique chat identifier.
    pub id: i64,
    #[serde(rename = "type")]
    /// The kind of chat.
    pub type_: ChatType,
}
