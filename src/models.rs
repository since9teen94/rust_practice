use log::debug;
use std::borrow::Cow;

use crate::establish_connection;

use super::schema::users;
use diesel::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

lazy_static! {
    static ref ONE_UPPER_CASE_CHAR: Regex = Regex::new(r"[A-Z]+").unwrap();
    static ref ONE_LOWER_CASE_CHAR: Regex = Regex::new(r"[a-z]+").unwrap();
    static ref ONE_NUMBER: Regex = Regex::new(r"\d+").unwrap();
    static ref ONE_NON_ALPHA_CHAR: Regex = Regex::new(r"\W+").unwrap();
    static ref NO_SPACES: Regex = Regex::new(r"^[^ ]+$").unwrap();
}

fn email_exists(value: &str) -> Result<(), ValidationError> {
    use super::schema::users::dsl::*;
    let mut conn = establish_connection();
    let email_exists = users
        .select(email)
        .filter(email.eq(value))
        .first::<String>(&mut conn);
    if email_exists.is_err() {
        return Err(ValidationError::new("invalid"));
    }
    Ok(())
}

fn email_unique(value: &str) -> Result<(), ValidationError> {
    use super::schema::users::dsl::*;
    let mut conn = establish_connection();
    let email_unique = users
        .select(email)
        .filter(email.eq(value))
        .limit(2)
        .load::<String>(&mut conn)
        .unwrap();
    if email_unique.len() > 1 {
        return Err(ValidationError::new("invalid"));
    }
    Ok(())
}

fn custom_email_login_validator(value: &str) -> Result<(), ValidationError> {
    if let Err(e) = email_exists(value) {
        return Err(e);
    };
    if let Err(e) = email_unique(value) {
        return Err(e);
    };
    Ok(())
}

pub fn compare_password(password_one: &str, password_two: &str) -> Result<(), ValidationError> {
    if password_one != password_two {
        let mut invalid_password = ValidationError::new("must_match");
        invalid_password.message = Some(Cow::from("Invalid password"));
        //invalid_password.code = Cow::from("must_match");
        debug!("{invalid_password}");
        return Err(invalid_password);
    }
    Ok(())
}

fn password_login_validator(value: &str) -> Result<(), ValidationError> {
    //use super::schema::users::dsl::*;
    //let mut conn = establish_connection();
    //let db_password = users
    //.select(password)
    //.filter((password.eq(value), email.equal()))
    //.limit(2)
    //.load::<String>(&mut conn)
    //.unwrap();
    //if email_unique.len() > 1 {
    //return Err(ValidationError::new("invalid"));
    //}
    Ok(())
}

#[derive(Deserialize, Validate, Debug)]
pub struct UserLogin {
    #[validate(length(min = 1))]
    #[validate(email)]
    #[validate(custom(function = "custom_email_login_validator", message = "Invalid email"))]
    pub email: String,
    #[validate(length(min = 1))]
    #[validate(custom(function = "password_login_validator"))]
    pub password: String,
}

#[derive(Deserialize, Validate, Debug)]
pub struct UserRegistration {
    #[validate(length(min = 1, message = "First name required"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "Last name required"))]
    pub last_name: String,
    #[validate(length(min = 1, message = "Email required"))]
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, message = "Password required"))]
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[validate(must_match(other = "confirm_password", message = "Passwords must match"))]
    #[validate(regex(
        path = "ONE_UPPER_CASE_CHAR",
        message = "Password must contain at least one uppercase character"
    ))]
    #[validate(regex(
        path = "ONE_LOWER_CASE_CHAR",
        message = "Password must contain at least one lowercase character"
    ))]
    #[validate(regex(
        path = "ONE_NUMBER",
        message = "Password must contain at least one number"
    ))]
    #[validate(regex(
        path = "ONE_NON_ALPHA_CHAR",
        message = "Password must contain at least one special character"
    ))]
    pub password: String,
    #[validate(length(min = 1, message = "Password confirmation required"))]
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[validate(must_match(other = "password", message = "Passwords must match"))]
    #[validate(regex(
        path = "ONE_UPPER_CASE_CHAR",
        message = "Password must contain at least one uppercase character"
    ))]
    #[validate(regex(
        path = "ONE_LOWER_CASE_CHAR",
        message = "Password must contain at least one lowercase character"
    ))]
    #[validate(regex(
        path = "ONE_NUMBER",
        message = "Password must contain at least one number"
    ))]
    #[validate(regex(
        path = "ONE_NON_ALPHA_CHAR",
        message = "Password must contain at least one special character"
    ))]
    #[validate(regex(path = "NO_SPACES", message = "Password must not contain spaces"))]
    pub confirm_password: String,
}

pub fn register(registration: UserRegistration) -> NewUser {
    let UserRegistration {
        first_name,
        last_name,
        email,
        password,
        ..
    } = registration;
    NewUser {
        first_name,
        last_name,
        email,
        password,
    }
}
