use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUserCommand {
    pub name: String,
    pub email: String,
}