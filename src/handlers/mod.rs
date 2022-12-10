use serde::{Serialize, Deserialize};

pub mod load_messages;
pub mod login;
pub mod post_message;
pub mod register;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub nickname: String,
    pub password: String,
}

impl User {
    pub fn check_nickname(nick: &String) -> bool {
        nick.chars().all(char::is_alphanumeric)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    text: String,
}
