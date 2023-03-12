use diesel::{Insertable, Queryable};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use super::schema::{students, teachers, workers};
use super::schema::students::dsl::students as student_dsl;
use super::schema::teachers::dsl::teachers as teacher_dsl;
use super::schema::workers::dsl::workers as worker_dsl;



#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name= "students" ]
pub struct Student {
    pub id: i32,
    pub fio: String,
    pub email: String,
    pub phone: String,
    pub group_id: i32
}

impl Student {
    pub fn list(conn: &PgConnection) -> Vec<Self> {
        student_dsl.load::<Student>(conn).expect("Error loading users")
    }

    pub fn by_id(id: &i32, conn: &PgConnection) -> Option<Self> {
        if let Ok(record) = student_dsl.find(id).get_result::<Student>(conn) {
            Some(record)
        } else {
            None
        }
    }

    pub fn by_email(email_str: &str, conn: &PgConnection) -> Option<Self> {
        use super::schema::students::dsl::email;
        if let Ok(record) = student_dsl.filter(email.eq(email_str)).first::<Student>(conn) {
            Some(record)
        } else {
            None
        }
    }
    // Нижняя часть нужна, чтобы вносить элемент в бд
    // pub fn create(username: Option<&str>, password: Option<&str>, conn: &PgConnection) -> Option<Self> {
    //     let new_id = Uuid::new_v4().to_hyphenated().to_string();
    //
    //     if username.is_some() {
    //         if let Some(user) = Self::by_username(&username.unwrap(), conn) {
    //             return Some(user)
    //         }
    //     }
    //
    //     let new_user = Self::new_user_struct(&new_id, username, password);
    //
    //     diesel::insert_into(user_dsl)
    //         .values(&new_user)
    //         .execute(conn)
    //         .expect("Error saving new user");
    //     Self::by_id(&new_id, conn)
    // }
    //
    // fn new_user_struct(id: &str, username: Option<&str>, password: Option<&str>) -> Self {
    //     User {
    //         id: id.into(),
    //         username: username.map(Into::into),
    //         password: password.map(Into::into),
    //         phone: None,
    //         email: None,
    //         avatar: None,
    //     }
    // }
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name= "teachers" ]
pub struct Teacher {
    pub id: i32,
    pub fio: String,
    pub email: String,
    pub phone: String
}

impl Teacher {
    pub fn list(conn: &PgConnection) -> Vec<Self> {
        teacher_dsl.load::<Teacher>(conn).expect("Error loading users")
    }

    pub fn by_id(id: &i32, conn: &PgConnection) -> Option<Self> {
        if let Ok(record) = teacher_dsl.find(id).get_result::<Teacher>(conn) {
            Some(record)
        } else {
            None
        }
    }

    pub fn by_email(email_str: &str, conn: &PgConnection) -> Option<Self> {
        use super::schema::teachers::dsl::email;
        if let Ok(record) = teacher_dsl.filter(email.eq(email_str)).first::<Teacher>(conn) {
            Some(record)
        } else {
            None
        }
    }
}


#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name= "workers" ]
pub struct Worker {
    pub id: i32,
    pub fio: String,
    pub email: String,
    pub phone: String,
    pub position_office: String,
    pub section_id: i32
}

impl Worker {
    pub fn list(conn: &PgConnection) -> Vec<Self> {
        worker_dsl.load::<Worker>(conn).expect("Error loading users")
    }

    pub fn by_id(id: &i32, conn: &PgConnection) -> Option<Self> {
        if let Ok(record) = worker_dsl.find(id).get_result::<Worker>(conn) {
            Some(record)
        } else {
            None
        }
    }

    pub fn by_email(email_str: &str, conn: &PgConnection) -> Option<Self> {
        use super::schema::workers::dsl::email;
        if let Ok(record) = worker_dsl.filter(email.eq(email_str)).first::<Worker>(conn) {
            Some(record)
        } else {
            None
        }
    }
}

// #[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
// #[table_name = "chats" ]
// pub struct Chats {
//     pub id: i32,
//     pub title: Option<String>,
//     pub id_creator: Option<String>,
//     pub picture: Option<Vec<u8>>,
//     pub is_dialog: Option<bool>
// }
//
// #[derive(Debug, Serialize, Insertable)]
// #[table_name = "chats" ]
// pub struct Chats1 {
//     pub id: Option<i32>,
//     pub title: Option<String>,
//     pub id_creator: Option<String>,
//     pub picture: Option<Vec<u8>>,
//     pub is_dialog: Option<bool>
// }
//
// impl Chats {
//     pub fn list(conn: &PgConnection) -> Vec<Self> {
//         chats_dsl.load::<Chats>(conn).expect("Error loading users")
//     }
//
//     pub fn by_id(id: i32, conn: &PgConnection) -> Option<Self> {
//         if let Ok(record) = chats_dsl.find(id).get_result::<Chats>(conn) {
//             Some(record)
//         } else {
//             None
//         }
//     }
//
//     pub fn create(title: Option<&str>, id_creator: Option<&String>, is_dialog: Option<bool>, conn: &PgConnection){
//
//         let new_chat = Self::new_chat_struct(title, id_creator, is_dialog);
//
//         diesel::insert_into(chats_dsl)
//             .values(&new_chat)
//             .execute(conn)
//             .expect("Error saving new user");
//
//     }
//     pub fn upd(title1: &String, chat_id: &i32, conn: &PgConnection) {
//         use super::schema::chats::dsl::{id, title};
//         let upd_row = diesel::update(chats_dsl.filter(id.eq(chat_id)))
//             .set(title.eq(title1)).get_result::<Chats>(conn);
//     }
//
//     fn new_chat_struct(title: Option<&str>, id_creator: Option<&String>, is_dialog: Option<bool>) -> Chats1 {
//         Chats1 {
//             id: None,
//             title: title.map(Into::into),
//             id_creator: id_creator.map(Into::into),
//             picture: None,
//             is_dialog: is_dialog.map(Into::into)
//         }
//     }
// }
//
// #[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
// #[table_name = "users_chats" ]
// pub struct UsersChats {
//     user_id: String,
//     pub chat_id: i32
// }
//
// impl UsersChats {
//      pub fn list(conn: &PgConnection) -> Vec<Self> {
//         users_chats_dsl.load::<UsersChats>(conn).expect("Error loading users")
//     }
//
//     pub fn by_id(id: String, conn: &PgConnection) -> Option<Vec<Self>> {
//         if let Ok(record) = users_chats_dsl.filter(super::schema::users_chats::user_id.eq(id.as_str())).get_results::<UsersChats>(conn) {
//             Some(record)
//         } else {
//             None
//         }
//     }
//
//     pub fn add(user_id: String, chat_id: i32, conn: &PgConnection) {
//
//         let making = Self::new_user_chat(user_id, chat_id);
//
//         diesel::insert_into(users_chats_dsl)
//             .values(&making)
//             .execute(conn)
//             .expect("Error saving new user");
//
//     }
//
//     fn new_user_chat(user_id: String, chat_id: i32) -> Self {
//         UsersChats {
//             user_id,
//             chat_id
//         }
//     }
// }
//
// #[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
// #[table_name = "message_content"]
// pub struct MessageContent {
//     pub id: i32,
//     pub content: Option<String>,
//     type_content: Option<String>
// }
//
// #[derive(Debug, Serialize, Insertable)]
// #[table_name = "message_content"]
// pub struct MessageContent1 {
//     content: Option<String>,
//     type_content: Option<String>
// }
//
// impl MessageContent {
//      pub fn list(conn: &PgConnection) -> Vec<Self> {
//         message_content_dsl.load::<MessageContent>(conn).expect("Error loading users")
//     }
//
//     pub fn by_id(id: i32, conn: &PgConnection) -> Option<Self> {
//         if let Ok(record) = message_content_dsl.filter(super::schema::message_content::id.eq(id)).get_result::<MessageContent>(conn) {
//             Some(record)
//         } else {
//             None
//         }
//     }
//
//     pub fn push(content: Option<&String>,  type_content: Option<String>, conn: &PgConnection) {
//
//         let message = Self::new_message_content(content, type_content);
//
//         diesel::insert_into(message_content_dsl)
//             .values(&message)
//             .execute(conn)
//             .expect("Error saving new user");
//
//     }
//
//     fn new_message_content(content: Option<&String>, type_content: Option<String>) -> MessageContent1 {
//         MessageContent1 {
//             content: content.map(Into::into),
//             type_content: type_content.map(Into::into)
//         }
//     }
// }
//
// #[derive(Debug, Deserialize, Queryable, Serialize, Insertable)]
// #[table_name = "messages"]
// pub struct Messages {
//     pub id: i32,
//     pub chat_id: Option<i32>,
//     pub sender_id: Option<String>,
//     pub date_send: Option<chrono::NaiveDateTime>,
//     pub content_id: Option<i32>
// }
//
// #[derive(Debug, Serialize, Insertable)]
// #[table_name = "messages"]
// pub struct Messages1 {
//     chat_id: Option<i32>,
//     sender_id: Option<String>,
//     date_send: Option<chrono::NaiveDateTime>,
//     content_id: Option<i32>
// }
//
// impl Messages {
//      pub fn list(conn: &PgConnection) -> Vec<Self> {
//         messages_dsl.load::<Messages>(conn).expect("Error loading users")
//     }
//
//     pub fn by_id(id: i32, conn: &PgConnection) -> Option<Vec<Self>> {
//         if let Ok(record) = messages_dsl.filter(super::schema::messages::chat_id.eq(id)).get_results::<Messages>(conn) {
//             Some(record)
//         } else {
//             None
//         }
//     }
//
//
//     pub fn push(chat_id: Option<i32>, sender_id: Option<String>, date_send: Option<chrono::NaiveDateTime>, content_id: Option<i32>, conn: &PgConnection) {
//
//         let message = Self::new_message(chat_id, sender_id, date_send, content_id);
//
//         diesel::insert_into(messages_dsl)
//             .values(&message)
//             .execute(conn)
//             .expect("Error saving new user");
//
//     }
//
//     fn new_message(chat_id: Option<i32>, sender_id: Option<String>, date_send: Option<chrono::NaiveDateTime>, content_id:  Option<i32>,) -> Messages1 {
//         Messages1 {
//             chat_id: chat_id.map(Into::into),
//             sender_id: sender_id.map(Into::into),
//             date_send: date_send.map(Into::into),
//             content_id: content_id.map(Into::into)
//         }
//     }
// }
//
