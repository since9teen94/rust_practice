use crate::establish_connection;
use crate::schema::users;
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use diesel::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use validator::{Validate, ValidationError};

lazy_static! {
    static ref ONE_UPPER_CASE_CHAR: Regex = Regex::new(r"[A-Z]+").unwrap();
    static ref ONE_LOWER_CASE_CHAR: Regex = Regex::new(r"[a-z]+").unwrap();
    static ref ONE_NUMBER: Regex = Regex::new(r"\d+").unwrap();
    static ref ONE_NON_ALPHA_CHAR: Regex = Regex::new(r"\W+").unwrap();
    static ref NO_SPACES: Regex = Regex::new(r"^[^ ]+$").unwrap();
}

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
    #[validate(
        custom(function = "custom_registration_email_validator",),
        email,
        length(min = 1, message = "Email required")
    )]
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

fn custom_registration_email_validator(value: &str) -> Result<(), ValidationError> {
    email_count(value, 0)
}

fn custom_login_validator(user_login: &UserLogin) -> Result<(), ValidationError> {
    let UserLogin { email, password } = user_login;
    if email_count(email, 1).is_err() || custom_login_password_validator(password, email).is_err() {
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

fn email_count(value: &str, count: usize) -> Result<(), ValidationError> {
    use super::schema::users::dsl::*;
    let mut conn = establish_connection();
    let email_unique = users
        .select(email)
        .filter(email.eq(value))
        .limit(2)
        .load::<String>(&mut conn);
    if email_unique.is_err() || email_unique.unwrap().len() != count {
        return Err(ValidationError::new("email"));
    };
    Ok(())
}

fn password_hash_checker(
    password: &str,
    password_hash: &str,
) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(password_hash)?;
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
}

#[derive(Serialize)]
pub struct LogRegForm {
    title: String,
    action: String,
    fields: Vec<LogRegFormField>,
}

impl LogRegForm {
    pub fn new(title: &str, action: &str) -> LogRegForm {
        let login_fields = vec![
            //LogRegFormField::new("first_name", "First Name", "first_name"),
            //LogRegFormField::new("last_name", "Last Name", "last_name"),
            LogRegFormField::new("email", "Email", "email"),
            LogRegFormField::new("password", "Password", "password"),
            //LogRegFormField::new("confirm_password", "Confirm Password", "confirm_password"),
        ];
        let register_fields = &login_fields; //.push(LogRegFormField::new( "first_name", "First Name", "first_name",));
        register_fields.push(LogRegFormField::new(
            "first_name",
            "First Name",
            "first_name",
        ));
        //vec![
        //LogRegFormField::new("first_name", "First Name", "first_name"),
        //LogRegFormField::new("last_name", "Last Name", "last_name"),
        //LogRegFormField::new("email", "Email", "email"),
        //LogRegFormField::new("password", "Password", "password"),
        //LogRegFormField::new("confirm_password", "Confirm Password", "confirm_password"),
        //]
        let fields = match title {
            "Log In" => login_fields,
            "Register" => vec![
                LogRegFormField::new("first_name", "First Name", "first_name"),
                LogRegFormField::new("last_name", "Last Name", "last_name"),
                LogRegFormField::new("email", "Email", "email"),
                LogRegFormField::new("password", "Password", "password"),
                LogRegFormField::new("confirm_password", "Confirm Password", "confirm_password"),
            ],
            _ => vec![],
        };

        LogRegForm {
            title: String::from(title),
            action: String::from(action),
            fields,
        }
    }
}

#[derive(Serialize)]
pub struct LogRegFormField {
    id: String,
    text: String,
    field_type: String,
}

impl LogRegFormField {
    pub fn new(id: &str, text: &str, field_type: &str) -> LogRegFormField {
        LogRegFormField {
            id: String::from(id),
            text: String::from(text),
            field_type: String::from(field_type),
        }
    }
}
