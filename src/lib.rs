pub mod models;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
pub mod schema;
//use argon2::{
//password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
//Argon2,
//};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn password_hasher(password_str: &str) -> Result<String, argon2::password_hash::Error> {
    //let password = b"hunter42"; // Bad password; don't actually use!
    let password = password_str.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password, &salt)?.to_string();

    //let parsed_hash = PasswordHash::new(&password_hash)?;
    //assert!(Argon2::default()
    //.verify_password(password, &parsed_hash)
    //.is_ok());
    //println!("{password_hash}");
    //println!("aoeuaoeuaoeu: {parsed_hash}");
    Ok(password_hash)
}

//TODO fix hashing implementation
pub fn password_hash_checker(password_hash: &str) -> Result<(), &'static str> {
    let parsed_hash = PasswordHash::new(&password_hash)?;
    assert!(Argon2::default()
        .verify_password(password, &parsed_hash)
        .is_ok());
}
