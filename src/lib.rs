pub mod models;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use std::borrow::Cow;
pub mod schema;
use actix_web::{error, HttpResponse, Responder};
use diesel::{insert_into, pg::PgConnection, prelude::*};
use dotenvy::dotenv;
use lazy_static::lazy_static;
use models::{NewUser, UserRegistration};
use std::env;
use tera::{Context, Tera};
use validator::ValidationError;

lazy_static! {
    pub static ref TEMPLATES: Tera = Tera::new("templates/*").unwrap();
}

pub async fn not_allowed() -> impl Responder {
    HttpResponse::MethodNotAllowed()
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn render(file: &str, context: Context) -> Result<HttpResponse, actix_web::Error> {
    match TEMPLATES.render(file, &context) {
        Ok(t) => Ok(HttpResponse::Ok().body(t)),
        Err(e) => Err(error::ErrorInternalServerError(e)),
    }
}

pub fn register(user: UserRegistration) -> Result<(), ValidationError> {
    let UserRegistration {
        first_name,
        last_name,
        email,
        password,
        confirm_password: _,
    } = user;
    let hashed_password = password_hasher(&password).unwrap();
    let new_user = NewUser {
        first_name,
        last_name,
        email,
        password: hashed_password,
    };
    create_user(new_user)
}

fn password_hasher(password_str: &str) -> Result<String, argon2::password_hash::Error> {
    let password = password_str.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password, &salt)?.to_string();
    Ok(password_hash)
}

fn create_user(new_user: NewUser) -> Result<(), ValidationError> {
    use schema::users::dsl::*;
    let conn = &mut establish_connection();
    if insert_into(users).values(new_user).execute(conn).is_err() {
        let mut registration_error = ValidationError::new("registration_error");
        registration_error.message = Some(Cow::Borrowed("An error occured during registration"));
        return Err(registration_error);
    };
    Ok(())
}
