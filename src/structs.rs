use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
 pub struct AuthData {
   pub email: String,
   pub type_lk: String
 }

 #[derive(Serialize, Deserialize, Debug)]
 pub struct Token {
   pub token: String
 }