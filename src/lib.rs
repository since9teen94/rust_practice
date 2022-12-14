pub mod forms;
pub mod models;
pub mod routes;
pub mod schema;
use actix_web::{http::StatusCode, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use diesel::{insert_into, pg::PgConnection, prelude::*};
use dotenvy::dotenv;
use lazy_static::lazy_static;
use models::{NewUser, UserRegistration};
use std::borrow::Cow;
use std::env;
use tera::{Context, Tera};
use validator::ValidationError;

lazy_static! {
    static ref TEMPLATES: Tera = Tera::new("templates/*").unwrap();
    pub static ref JSON: &'static str = "application/json";
    pub static ref HTML: &'static str = "text/html";
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

//pub fn render(file: &str, context: Context) -> Result<HttpResponse, actix_web::Error> {
//match TEMPLATES.render(file, &context) {
//Ok(t) => Ok(HttpResponse::Ok().body(t)),
//Err(e) => Err(error::ErrorInternalServerError(e)),
//}
//}

pub fn render(file: &str, context: Context) -> HttpResponse {
    let template = TEMPLATES.render(file, &context).unwrap();
    HttpResponse::Ok().body(template)
}

pub async fn register(user: UserRegistration) -> Result<(), ValidationError> {
    let UserRegistration {
        first_name,
        last_name,
        email,
        _password,
        _confirm_password,
    } = user;
    let hashed_password = password_hasher(&_password.unwrap()).await;
    let new_user = NewUser {
        first_name: first_name.unwrap(),
        last_name: last_name.unwrap(),
        email: email.unwrap(),
        password: hashed_password.unwrap(),
    };
    create_user(new_user).await
}

async fn password_hasher(password_str: &str) -> Result<String, argon2::password_hash::Error> {
    let password = password_str.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password, &salt)?.to_string();
    Ok(password_hash)
}

async fn create_user(new_user: NewUser) -> Result<(), ValidationError> {
    use schema::users::dsl::*;
    let conn = &mut establish_connection();
    if insert_into(users).values(new_user).execute(conn).is_err() {
        let mut registration_error = ValidationError::new("registration_error");
        registration_error.message = Some(Cow::Borrowed("An error occured during registration"));
        return Err(registration_error);
    };
    Ok(())
}

pub fn response(
    http_status_code: u16,
    content_type: &'static str,
    body: Option<String>,
) -> HttpResponse {
    if body.is_none() {
        let content_type = format!("{content_type}; charset=utf-8");
        return HttpResponse::build(StatusCode::from_u16(http_status_code).unwrap())
            .content_type(content_type)
            .finish();
    };
    let content_type = format!("{content_type}; charset=utf-8");
    HttpResponse::build(StatusCode::from_u16(http_status_code).unwrap())
        .content_type(content_type)
        .body(body.unwrap())
}

pub async fn not_allowed() -> impl Responder {
    HttpResponse::build(StatusCode::METHOD_NOT_ALLOWED)
        .content_type("text/html; charset=utf-8")
        .body("<h1>405 Not Allowed</h1>")
}

pub async fn not_found() -> impl Responder {
    HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type("text/html; charset=utf-8")
        .body("<h1>404</h1><p>Page Not Found</p>")
}
