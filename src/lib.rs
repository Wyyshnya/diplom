pub mod database;
pub mod structs;
mod auth_handler;

#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;


use std::fs;
use std::io::Write;
use actix_cors::Cors;
use actix_multipart::{Field, Multipart};
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{App, Error, http, HttpRequest, HttpResponse, HttpServer, web};
use futures::{StreamExt, TryStreamExt};
use crate::database::DbPool;
use crate::database::models::Chats;
use crate::structs::AuthData;

pub struct MessageApp {
    port: u16,
}

impl MessageApp {
    pub fn new(port: u16) -> Self {

        MessageApp { port }
    }
    //TODO сделать профили пользователей, доделать выпадающий список для создания чата
    pub async fn run(&self) -> std::io::Result<()> {
        let conn_pool = database::establish_connection();
        println!("Starting http server: my_ip:{}", self.port);
        HttpServer::new(move || {
            let cors = Cors::default()
              .allow_any_origin()
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);
            App::new()
                .app_data(web::Data::new(conn_pool.clone()))
                .wrap(IdentityService::new(CookieIdentityPolicy::new(&[0; 32])
                          .name("auth-cookie")
                          .secure(false))).wrap(cors)
                .route("/api/sign_in", web::post().to(signin))
                .route("/api/conversations/{id}", web::get().to(conversations))
                .route("api/messages/{id}", web::get().to(messages))
                .route("api/decode_jwt", web::post().to(decoding))
                .route("api/message_post/{id}", web::post().to(message_post))
                .route("api/get_chat_name/{id_chat}", web::get().to(get_chat_name))
                .route("api/add_conversation", web::post().to(add_conv))
                .route("api/get_by_name", web::post().to(get_by_name))
                .route("api/get_profile/{id}", web::get().to(get_profile))
                .route("api/get_group_by_id/{id}", web::get().to(get_group_by_id))
                // .service(web::resource("api/message_probe").route(web::post().to(message_probe)))
                // .service(signin)
                // .wrap(actix_web::middleware::Logger::default()) // добавляем логгер
                // .wrap(actix_web::middleware::NormalizePath::default()) // добавляем нормализатор пути
        })
        .bind(("127.0.0.1", self.port))?
        .workers(8).run().await
    }
}

// #[post("api/sign_in")]
async fn signin(id: Identity, req: HttpRequest, data: web::Json<AuthData>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();
    let email = &*data.email;
    auth_handler::login(id, email, conn).await
}


// #[get("api/conversations/{id}")]
async fn conversations(req: HttpRequest, pool: web::Data<DbPool>) -> HttpResponse {
    let id: String = req.match_info().get("id").unwrap().parse().unwrap();
    let conn = pool.get().unwrap();
    let user = auth_handler::decode_jwt(&id).unwrap();
    let ids = database::models::UsersChats::by_id(user.id, &conn);
    let mut chats = vec![];
    let mut contents = vec![];
    for i in ids.unwrap() {
        chats.push(database::models::Chats::by_id(i.chat_id, &conn).unwrap());
        let mut id = database::models::Messages::by_id(i.chat_id, &conn).unwrap();
        match id.pop() {
            Some(i) => {
                let content = database::models::MessageContent::by_id(i.content_id, &conn);
                contents.push(content.unwrap().content);
            },
            None => contents.push("None".to_string())
        }
    };
    let send = structs::SendChats {item: chats, last_messages: contents};
    HttpResponse::Ok().json(send)
}

// #[get("api/messages/{id}")]
async fn messages(req: HttpRequest, pool: web::Data<DbPool>) -> HttpResponse {
    // println!("{:?}", &req.headers().get("authorization"));
    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let conn = pool.get().unwrap();
    let messages = database::models::Messages::by_id(id, &conn).unwrap();
    let mut send = vec![];
    for message in &messages {
        let content = database::models::MessageContent::by_id(message.content_id, &conn).unwrap();
        let mess = structs::Message {
            id: message.id,
            sender_id: message.sender_id,
            date_send: message.date_send,
            content,
        };
        send.push(mess);
    }
    // let send = SendChats {item: chats};
    HttpResponse::Ok().json(send)
}

// #[post("api/decode_jwt")]
async fn decoding(req: HttpRequest, data: web::Json<structs::Token>) -> HttpResponse {
    let user = auth_handler::decode_jwt(&data.token);
    HttpResponse::Ok().json(user.unwrap())
}

// #[post("api/message_post/{id}")]
// async fn message_post(req: HttpRequest, data: web::Json<structs::PostData>, pool: web::Data<DbPool>) -> HttpResponse {
//     let conn = pool.get().unwrap();
//     let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
//     let user = auth_handler::decode_jwt(&data.id_user);
//     database::models::MessageContent::push(&data.message, "text".to_string(), &conn);
//     let mut last_id = database::models::MessageContent::list(&conn);
//     database::models::Messages::push(id, user.unwrap().id,
//                                chrono::Local::now().naive_local(), last_id.pop().unwrap().id, &conn);
//     HttpResponse::Ok().json("successful")
// }

async fn message_post(req: HttpRequest, mut payload: Multipart, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    // Обрабатываем поля формы
    let conn = pool.get().unwrap();
    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let mut user;
    while let Some(item) = payload.next().await {
        let mut field = item?;

        let content_disposition = field.content_disposition().unwrap();
        let name = content_disposition.get_name().unwrap().to_string();

        match content_disposition.get_filename() {
            Some(filename) => {
                // Если поле - файл, обрабатываем его асинхронно
                let filepath = format!("/home/wyyshnya/RustProjects/aci_diplom/{}", filename);
                let mut f = web::block(|| fs::File::create(filepath))
                    .await.unwrap();

                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap();
                }
            }
            None => {
                // Если это обычное поле формы
                let value = field
                    .map_ok(|bytes| bytes.to_vec())
                    .try_concat()
                    .await
                    .unwrap();
                let string_value = String::from_utf8_lossy(&value).to_string();
                if name == "id_user".to_string() {
                    user = auth_handler::decode_jwt(&string_value);
                    let field_ = payload.next().await.unwrap()?;
                    let value = field_
                        .map_ok(|bytes| bytes.to_vec())
                        .try_concat()
                        .await
                        .unwrap();
                    let string_value = String::from_utf8_lossy(&value).to_string();
                    database::models::MessageContent::push(&string_value, "text".to_string(), &conn);
                    let mut last_id = database::models::MessageContent::list(&conn);
                    database::models::Messages::push(id, user.unwrap().id,
                                chrono::Local::now().naive_local(), last_id.pop().unwrap().id, &conn);
                }
            }
        }
    }
    Ok(HttpResponse::Ok().into())
}

////TODO axios-cache-adapter
// #[get("api/get_chat_name/{id_chat}")]
async fn get_chat_name(req: HttpRequest, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();
    let id_chat = req.match_info().get("id_chat").unwrap().parse().unwrap();
    let chat_name = database::models::Chats::by_id(id_chat, &conn).unwrap();
    let title = structs::TitleChat {title_chat:chat_name.title};
    HttpResponse::Ok().json(title)
}

// #[post("api/add_conversation")]
//TODO сделать выбор с кем создать чат, ЕСЛИ студент - со студентами и преподом,
// ЕСЛИ препод - чат с группой. В общем и целом - сделать выборку из бд.
async fn add_conv(req: HttpRequest, data: web::Json<structs::ConvData>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();
    let user = auth_handler::decode_jwt(&data.user_token).unwrap();
    Chats::create(&data.chat_name, true, &conn);
    let chat_list = Chats::list(&conn).pop();
    database::models::UsersChats::add(user.id, chat_list.as_ref().unwrap().id, &conn);
    HttpResponse::Ok().json("successful")
}

async fn get_by_name(req: HttpRequest, data: web::Json<structs::SearchNames>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();
    let user_list = database::models::User::by_name(&data.name, &conn);

    HttpResponse::Ok().json(structs::ListUsers{user_comp: user_list.unwrap()})
}

// #[post("api/change_chat_title")]
// async fn chng_title(req: HttpRequest, data: web::Json<TitleData>, pool: web::Data<db_pool>) -> HttpResponse {
//     let conn = pool.get().unwrap();
//     Chats::upd(&data.chat_name, &data.chat_id, &conn);
//     HttpResponse::Ok().json("successful")
// }

async fn get_profile(req: HttpRequest,  pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();
    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let user = database::models::User::by_id(&id, &conn).unwrap();

    HttpResponse::Ok().json(user)
}

async fn get_group_by_id(req: HttpRequest, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();
    let id: i32 = req.match_info().get("id").unwrap().parse().unwrap();
    let user = database::models::Groups::by_id(id, &conn).unwrap();

    HttpResponse::Ok().json(user)
}

