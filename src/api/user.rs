use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
/// The subset of a Telegram user or bot currently modeled by the crate.
pub struct User {
    /// The unique user or bot identifier.
    pub id: i64,
    /// The first name.
    pub first_name: String,
    /// The last name, when present.
    pub last_name: Option<String>,
    /// The username, when present.
    pub username: Option<String>,
}
