use crate::{establish_connection, password_hash_checker};

use super::{password_hasher, schema::users};
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

#[derive(Debug, Validate, Deserialize)]
#[validate(schema(
    function = "custom_login_validator",
    message = "Invalid Credentials",
    skip_on_field_errors = false
))]
pub struct UserLogin {
    #[validate(length(min = 1, message = "Email required"), email)]
    pub email: String,
    #[validate(length(min = 1, message = "Password Required"))]
    pub password: String,
}

#[derive(Deserialize, Validate, Debug)]
pub struct UserRegistration {
    #[validate(length(min = 1, message = "First name required"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "Last name required"))]
    pub last_name: String,
    #[validate(email, length(min = 1, message = "Email required"))]
    pub email: String,
    #[validate(
        regex(
            path = "ONE_UPPER_CASE_CHAR",
            message = "Password must contain at least one uppercase character"
        ),
        regex(
            path = "ONE_LOWER_CASE_CHAR",
            message = "Password must contain at least one lowercase character"
        ),
        regex(
            path = "ONE_NUMBER",
            message = "Password must contain at least one number"
        ),
        regex(
            path = "ONE_NON_ALPHA_CHAR",
            message = "Password must contain at least one special character"
        ),
        regex(path = "NO_SPACES", message = "Password must not contain spaces"),
        must_match(other = "confirm_password", message = "Passwords must match"),
        length(min = 8, message = "Password must be at least 8 characters"),
        length(min = 1, message = "Password required")
    )]
    pub password: String,
    #[validate(
        length(min = 1, message = "Password confirmation required"),
        length(min = 8, message = "Password must be at least 8 characters"),
        must_match(other = "password", message = "Passwords must match"),
        regex(
            path = "ONE_UPPER_CASE_CHAR",
            message = "Password must contain at least one uppercase character"
        ),
        regex(
            path = "ONE_LOWER_CASE_CHAR",
            message = "Password must contain at least one lowercase character"
        ),
        regex(
            path = "ONE_NUMBER",
            message = "Password must contain at least one number"
        ),
        regex(
            path = "ONE_NON_ALPHA_CHAR",
            message = "Password must contain at least one special character"
        ),
        regex(path = "NO_SPACES", message = "Password must not contain spaces")
    )]
    pub confirm_password: String,
}

lazy_static! {
    static ref ONE_UPPER_CASE_CHAR: Regex = Regex::new(r"[A-Z]+").unwrap();
    static ref ONE_LOWER_CASE_CHAR: Regex = Regex::new(r"[a-z]+").unwrap();
    static ref ONE_NUMBER: Regex = Regex::new(r"\d+").unwrap();
    static ref ONE_NON_ALPHA_CHAR: Regex = Regex::new(r"\W+").unwrap();
    static ref NO_SPACES: Regex = Regex::new(r"^[^ ]+$").unwrap();
}

pub fn register(registration: UserRegistration) -> NewUser {
    let UserRegistration {
        first_name,
        last_name,
        email,
        password,
        ..
    } = registration;
    let hashed_password = password_hasher(&password).unwrap();
    NewUser {
        first_name,
        last_name,
        email,
        password: hashed_password,
    }
}

fn custom_login_email_validator(value: &str) -> Result<(), ValidationError> {
    use super::schema::users::dsl::*;
    //TODO flesh out this function in regards to differences tha occur with login / registration
    //fn email_exists(value: &str) -> Result<(), ValidationError> {
    //let mut conn = establish_connection();
    //let email_exists = users
    //.select(email)
    //.filter(email.eq(value))
    //.first::<String>(&mut conn);
    //if email_exists.is_err() {
    //return Err(ValidationError::new("invalid"));
    //}
    //Ok(())
    //}

    fn email_unique_and_exists(value: &str) -> Result<(), ValidationError> {
        let mut conn = establish_connection();
        let email_unique = users
            .select(email)
            .filter(email.eq(value))
            .limit(2)
            .load::<String>(&mut conn);
        if email_unique.is_err() {
            return Err(ValidationError::new("invalid"));
        };
        if email_unique.unwrap().len() != 1 {
            return Err(ValidationError::new("invalid"));
        }
        Ok(())
    }
    //email_exists(value)?;
    email_unique_and_exists(value)?;
    Ok(())
}

fn custom_login_validator(user_login: &UserLogin) -> Result<(), ValidationError> {
    let UserLogin { email, password } = user_login;
    if custom_login_email_validator(email).is_err() {
        return Err(ValidationError::new("invalid"));
    };
    if custom_login_password_validator(password, email).is_err() {
        return Err(ValidationError::new("invalid"));
    };
    Ok(())
}

fn custom_login_password_validator(value: &str, arg: &str) -> Result<(), ValidationError> {
    use super::schema::users::dsl::*;
    let mut conn = establish_connection();
    let db_password = users
        .select(password)
        .filter(email.eq(arg))
        .first::<String>(&mut conn);
    if db_password.is_err() {
        return Err(ValidationError::new("invalid"));
    };
    let password_check = password_hash_checker(value, &db_password.unwrap());
    if password_check.is_err() {
        return Err(ValidationError::new("invalid"));
    };
    Ok(())
}
