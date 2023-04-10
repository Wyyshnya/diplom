use actix_web::{web, Result, HttpResponse, HttpRequest, Error, FromRequest};
use actix_web::http::StatusCode;
use crate::database::models;
use actix_identity::{Identity, IdentityService, CookieIdentityPolicy, RequestIdentity};
use diesel::PgConnection;
use r2d2::PooledConnection;
use r2d2_diesel::ConnectionManager;
use serde::{Deserialize, Serialize};
use actix_web::error::ErrorBadRequest;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use crate::structs::Token;


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AuthUser {
    pub id: i32,
    pub fio: String,
    pub email: String,
    pub phone: String,
    pub role: String,
    pub position_office: Option<String>,
    pub group_id: Option<i32>,
    pub is_teacher: bool,
    exp: i64,
}

impl AuthUser {
    pub fn new(id: i32, fio: String,
               email: String, phone: String, role: String,
               position_office: Option<String>, group_id: Option<i32>,
               is_teacher: bool,
    ) -> Self {
        Self {
            id,
            fio,
            email,
            phone,
            role,
            position_office,
            group_id,
            is_teacher,
            exp: (Utc::now() + Duration::hours(10)).timestamp(),
        }
    }
}


pub async fn login(id: Identity, email: &str,
                   conn: PooledConnection<ConnectionManager<PgConnection>>) -> HttpResponse {
    match models::User::by_email(email, &conn) {
        Some(user) => {
            let role;
            if user.is_teacher {
                role = "teacher".to_string();
            }
            else if user.group_id.unwrap().is_positive() {
                role = "student".to_string();
            }
            else {
                role = "worker".to_string();
            }
            let auth_user = AuthUser::new(
                user.id, user.fio, user.email,
                user.phone, role, user.position_office, user.group_id, user.is_teacher);
            let token = create_jwt(auth_user);
            let tok = Token { token: token.as_ref().unwrap().to_string() };
            id.remember(token.unwrap());
            HttpResponse::Ok().json(tok)
        }
        _ => {
            HttpResponse::Ok().json("ERROR")
        }
    }
}


// Encode a json web token (JWT)
pub fn create_jwt(private_claim: AuthUser) -> Result<String, Error> {
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
    // TODO возможно, сделать мэтч на определение роли
    let decoding_key = DecodingKey::from_secret("secret_key".as_ref());
    decode::<AuthUser>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ErrorBadRequest(e.to_string()))
}

pub async fn logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::InternalServerError().json("Vishel")
}