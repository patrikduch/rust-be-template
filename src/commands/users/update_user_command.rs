use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateUserCommand {
    #[serde(skip_deserializing)] // You’ll set this manually from path param
    pub user_id: i32,
    pub name: String,
    pub email: String,
}