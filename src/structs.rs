use serde::{Deserialize, Serialize};
use crate::Chats;


#[derive(Serialize, Deserialize, Debug)]
pub struct SendChats {
    pub item: Vec<Chats>,
    pub last_messages: Vec<String>
}


#[derive(Serialize, Deserialize, Debug)]
 pub struct AuthData {
   pub email: String
 }

 #[derive(Serialize, Deserialize, Debug)]
 pub struct Token {
   pub token: String
 }