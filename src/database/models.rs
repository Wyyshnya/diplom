use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use super::schema::{users, users_chats, chats, messages, content_message};
use super::schema::users::dsl::users as users_dsl;
use super::schema::users_chats::dsl::users_chats as users_chats_dsl;
use super::schema::chats::dsl::chats as chats_dsl;
use super::schema::messages::dsl::messages as messages_dsl;
use super::schema::content_message::dsl::content_message as content_messages_dsl;



#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name= "users" ]
pub struct User {
    pub id: i32,
    pub fio: String,
    pub email: String,
    pub phone: String,
    pub position_office: Option<String>,
    pub group_id: Option<i32>,
    pub is_teacher: bool
}

impl User {
    pub fn list(conn: &PgConnection) -> Vec<Self> {
        users_dsl.load::<User>(conn).expect("Error loading users")
    }

    pub fn by_name(name: &String, conn: &PgConnection) -> Option<Vec<User>> {
        use super::schema::users::dsl::fio;
        if let Ok(record) = users_dsl.filter(fio.ilike(format!("%{}%", name))).get_results::<User>(conn) {
            Some(record)
        } else {
            None
        }
    }

    pub fn by_id(id: &i32, conn: &PgConnection) -> Option<Self> {
        if let Ok(record) = users_dsl.find(id).get_result::<User>(conn) {
            Some(record)
        } else {
            None
        }
    }

    pub fn by_email(email_str: &str, conn: &PgConnection) -> Option<Self> {
        use super::schema::users::dsl::email;
        if let Ok(record) = users_dsl.filter(email.eq(email_str)).first::<User>(conn) {
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
#[table_name = "users_chats" ]
pub struct UsersChats {
    user_id: i32,
    pub chat_id: i32
}

impl UsersChats {
     pub fn list(conn: &PgConnection) -> Vec<Self> {
        users_chats_dsl.load::<UsersChats>(conn).expect("Error loading users")
    }

    pub fn by_id(id: i32, conn: &PgConnection) -> Option<Vec<Self>> {
        if let Ok(record) = users_chats_dsl.filter(super::schema::users_chats::user_id.eq(id)).get_results::<UsersChats>(conn) {
            Some(record)
        } else {
            None
        }
    }

    pub fn add(user_id: i32, chat_id: i32, conn: &PgConnection) {

        let making = Self::new_user_chat(user_id, chat_id);

        diesel::insert_into(users_chats_dsl)
            .values(&making)
            .execute(conn)
            .expect("Error saving new user");

    }

    fn new_user_chat(user_id: i32, chat_id: i32) -> Self {
        UsersChats {
            user_id,
            chat_id
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "chats" ]
pub struct Chats {
    pub id: i32,
    pub title: String,
    pub is_dialog: bool
}

#[derive(Debug, Serialize, Insertable)]
#[table_name = "chats" ]
pub struct Chats1 {
    pub id: Option<i32>,
    pub title: String,
    pub is_dialog: bool
}

impl Chats {
    pub fn list(conn: &PgConnection) -> Vec<Self> {
        chats_dsl.load::<Chats>(conn).expect("Error loading users")
    }

    pub fn by_id(id: i32, conn: &PgConnection) -> Option<Self> {
        if let Ok(record) = chats_dsl.find(id).get_result::<Chats>(conn) {
            Some(record)
        } else {
            None
        }
    }

    pub fn create(title: &str, is_dialog: bool, conn: &PgConnection){

        let new_chat = Self::new_chat_struct(title,  is_dialog);

        diesel::insert_into(chats_dsl)
            .values(&new_chat)
            .execute(conn)
            .expect("Error saving new user");

    }
    pub fn upd(title1: &String, chat_id: &i32, conn: &PgConnection) {
        use super::schema::chats::dsl::{id, title};
        let upd_row = diesel::update(chats_dsl.filter(id.eq(chat_id)))
            .set(title.eq(title1)).get_result::<Chats>(conn);
    }

    fn new_chat_struct(title: &str, is_dialog: bool) -> Chats1 {
        Chats1 {
            id: None,
            title: title.to_string(),
            is_dialog
        }
    }
}


#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "content_message"]
pub struct MessageContent {
    pub id: i32,
    pub content: String,
    type_content: String
}

#[derive(Debug, Serialize, Insertable)]
#[table_name = "content_message"]
pub struct MessageContent1 {
    content: String,
    type_content: String
}

impl MessageContent {
     pub fn list(conn: &PgConnection) -> Vec<Self> {
        content_messages_dsl.load::<MessageContent>(conn).expect("Error loading users")
    }

    pub fn by_id(id: i32, conn: &PgConnection) -> Option<Self> {
        if let Ok(record) = content_messages_dsl.filter(super::schema::content_message::id.eq(id)).get_result::<MessageContent>(conn) {
            Some(record)
        } else {
            None
        }
    }

    pub fn push(content: &String,  type_content: String, conn: &PgConnection) {

        let message = Self::new_message_content(content.to_string(), type_content);

        diesel::insert_into(content_messages_dsl)
            .values(&message)
            .execute(conn)
            .expect("Error saving new user");

    }

    fn new_message_content(content: String, type_content: String) -> MessageContent1 {
        MessageContent1 {
            content,
            type_content
        }
    }
}

#[derive(Debug, Deserialize, Queryable, Serialize, Insertable)]
#[table_name = "messages"]
pub struct Messages {
    pub id: i32,
    pub chat_id: i32,
    pub sender_id: i32,
    pub date_send: chrono::NaiveDateTime,
    pub content_id: i32
}

#[derive(Debug, Serialize, Insertable)]
#[table_name = "messages"]
pub struct Messages1 {
    chat_id: i32,
    sender_id: i32,
    date_send: chrono::NaiveDateTime,
    content_id: i32
}

impl Messages {
     pub fn list(conn: &PgConnection) -> Vec<Self> {
        messages_dsl.load::<Messages>(conn).expect("Error loading users")
    }

    pub fn by_id(id: i32, conn: &PgConnection) -> Option<Vec<Self>> {
        if let Ok(record) = messages_dsl.filter(super::schema::messages::chat_id.eq(id)).get_results::<Messages>(conn) {
            Some(record)
        } else {
            None
        }
    }


    pub fn push(chat_id: i32, sender_id: i32, date_send: chrono::NaiveDateTime, content_id: i32, conn: &PgConnection) {

        let message = Self::new_message(chat_id, sender_id, date_send, content_id);

        diesel::insert_into(messages_dsl)
            .values(&message)
            .execute(conn)
            .expect("Error saving new user");

    }

    fn new_message(chat_id: i32, sender_id: i32, date_send: chrono::NaiveDateTime, content_id: i32,) -> Messages1 {
        Messages1 {
            chat_id,
            sender_id,
            date_send,
            content_id
        }
    }
}

