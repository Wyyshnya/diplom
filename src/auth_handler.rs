use actix_web::{web, Result, HttpResponse, HttpRequest, Error, FromRequest};
use actix_web::http::StatusCode;
use crate::database::models;
use actix_identity::{Identity, IdentityService, CookieIdentityPolicy, RequestIdentity};
use diesel::PgConnection;
use r2d2::PooledConnection;
use r2d2_diesel::ConnectionManager;
use serde::{Deserialize, Serialize};
use std::fs;
use actix_web::error::ErrorBadRequest;
use chrono::{Duration, Utc};
use uuid::Uuid;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use crate::database::schema::workers::section_id;
use crate::structs::Token;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AuthUser {
    pub auth_student: Option<AuthStudent>,
    pub auth_teacher: Option<AuthTeacher>,
    pub auth_worker: Option<AuthWorker>,
}

impl AuthUser {
    pub fn new(auth_student: Option<AuthStudent>, auth_teacher: Option<AuthTeacher>,
               auth_worker: Option<AuthWorker>) -> Self {
        Self {
            auth_student,
            auth_teacher,
            auth_worker
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AuthStudent {
    pub student_id: i32,
    pub student_fio: String,
    pub student_email: String,
    pub student_phone: String,
    pub student_group_id: i32,
    exp: i64,
}

impl AuthStudent {
    pub fn new(student_id: i32, student_fio: String, student_email: String, student_phone: String,
               student_group_id: i32) -> Self {
        Self {
            student_id,
            student_fio,
            student_email,
            student_phone,
            student_group_id,
            exp: (Utc::now() + Duration::hours(10)).timestamp(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AuthTeacher {
    pub teacher_id: i32,
    pub teacher_fio: String,
    pub teacher_email: String,
    pub teacher_phone: String,
    exp: i64,
}

impl AuthTeacher {
    pub fn new(teacher_id: i32, teacher_fio: String,
               teacher_email: String, teacher_phone: String) -> Self {
        Self {
            teacher_id,
            teacher_fio,
            teacher_email,
            teacher_phone,
            exp: (Utc::now() + Duration::hours(10)).timestamp(),
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AuthWorker {
    pub worker_id: i32,
    pub worker_fio: String,
    pub worker_email: String,
    pub worker_phone: String,
    pub worker_position_office: String,
    pub worker_section_id: i32,
    exp: i64,
}

impl AuthWorker {
    pub fn new(worker_id: i32, worker_fio: String,
               worker_email: String, worker_phone: String,
               worker_position_office: String, worker_section_id: i32) -> Self {
        Self {
            worker_id,
            worker_fio,
            worker_email,
            worker_phone,
            worker_position_office,
            worker_section_id,
            exp: (Utc::now() + Duration::hours(10)).timestamp(),
        }
    }
}


pub async fn login(id: Identity, email: &str, type_lk: &str,
                   conn: PooledConnection<ConnectionManager<PgConnection>>) -> HttpResponse {
    let mut auth_user = AuthUser::new(None, None, None);
    match type_lk {
        "student" => {
            match models::Student::by_email(email, &conn) {
                Some(student) => {
                    let auth_student = AuthStudent::new(
                        student.id, student.fio, student.email,
                        student.phone, student.group_id);
                    auth_user = AuthUser::new(
                        Some(auth_student), None, None,
                    );
                }
                _ => ()
            }
        }
        "teacher" => {
            match models::Teacher::by_email(email, &conn) {
                Some(teacher) => {
                    let auth_teacher = AuthTeacher::new(
                        teacher.id, teacher.fio, teacher.email,
                        teacher.phone);
                    auth_user = AuthUser::new(
                        None, Some(auth_teacher), None,
                    );
                }
                _ => ()
            }
        }
        "worker" => {
            match models::Worker::by_email(email, &conn) {
                Some(worker) => {
                    let auth_worker = AuthWorker::new(
                        worker.id, worker.fio, worker.email,
                        worker.phone, worker.position_office, worker.section_id);
                    auth_user = AuthUser::new(
                        None, None, Some(auth_worker),
                    );
                }
                _ => ()
            }
        }
        // _ => HttpResponse::InternalServerError().json("User doesn't exists")
        _ => ()
    }
    let token = create_jwt(auth_user);
    let tok = Token { token: token.as_ref().unwrap().to_string() };
    id.remember(token.unwrap());
    HttpResponse::Ok().json(tok)
}

// Encode a json web token (JWT)
pub fn create_jwt(private_claim: AuthUser) -> Result<String, Error> {
    // TODO посмотреть будет ли работать так, то есть зашифровывать структуры в структуре, если нет, то сделать мэтч
    let encoding_key = EncodingKey::from_secret("secret_key".as_ref());
    encode(
        &Header::default(),
        &private_claim,
        &encoding_key,
    )
        .map_err(|e| ErrorBadRequest(e.to_string()))
}

// Decode a json web token (JWT)
pub fn decode_jwt(token: &str) -> Result<AuthUser, Error> {
    let decoding_key = DecodingKey::from_secret("secret_key".as_ref());
    decode::<AuthUser>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ErrorBadRequest(e.to_string()))
}

pub async fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::InternalServerError().json("Vishel")
}