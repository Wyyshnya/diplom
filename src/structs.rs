use serde::{Deserialize, Serialize};
use crate::Chats;
use crate::database::models;
use crate::database::models::User;


#[derive(Serialize, Deserialize, Debug)]
pub struct SendChats {
    pub item: Vec<Chats>,
    pub last_messages: Vec<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AuthData {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: i32,
    pub sender_id: i32,
    pub date_send: chrono::NaiveDateTime,
    pub content: models::MessageContent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TitleChat {
    pub title_chat: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConvData {
    pub chat_name: String,
    pub user_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchNames {
    pub name: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ListUsers {
    pub user_comp: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Timesheet {
    pub type_search: String,
    pub number: String,
}