use actix_web::{
    web::{self, Form, Json},
    Either, HttpResponse, Responder,
};
use diesel::{insert_into, prelude::*};
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use validator::Validate;
use web_app::{establish_connection, models::*, password_hash_checker};

type RegisterNewUser = Either<Json<UserRegistration>, Form<UserRegistration>>;
type LoginUser = Either<Json<UserLogin>, Form<UserLogin>>;

pub async fn not_allowed() -> impl Responder {
    HttpResponse::MethodNotAllowed()
}

//TODO add static page / redirect with user form -> login / register
async fn index_get() -> impl Responder {
    "hello world!"
}

async fn login_get() -> impl Responder {
    "Hello login_get"
}

async fn login_post(login_data: LoginUser) -> impl Responder {
    let login = login_data.into_inner();
    if let Err(e) = login
        .validate()
        .map_err(|e| serde_json::to_string(&e).unwrap())
    {
        return e;
    };
    use web_app::schema::users::dsl::*;
    let mut conn = establish_connection();
    let hashed_password = users
        .select(password)
        .filter(email.eq(login.email))
        .first::<String>(&mut conn);

    if hashed_password.is_err() {
        return String::from("Invalid Credentials");
    };
    if password_hash_checker(&login.password, &hashed_password.unwrap()).is_err() {
        return String::from("Invalid Credentials");
    }
    String::from("User logged in successfully")
}

async fn register_get() -> impl Responder {
    "Hello register_get"
}

async fn register_post(registration_data: RegisterNewUser) -> impl Responder {
    use web_app::schema::users::dsl::*;

    let registration = registration_data.into_inner();
    match registration
        .validate()
        .map_err(|e| serde_json::to_string(&e).unwrap())
    {
        Ok(_) => {
            let conn = &mut establish_connection();
            let new_user = register(registration);

            if let Err(e) = insert_into(users).values(new_user).execute(conn) {
                return format!("Error registering user: {e}");
            };

            return String::from("User successfully registered");
        }
        Err(e) => {
            return format!("Error(s) encountered with user registration data: {e}");
        }
    }
}

pub fn index(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(index_get))
            .route(web::head().to(not_allowed)),
    )
    .service(
        web::resource("/login")
            .route(web::get().to(login_get))
            .route(web::post().to(login_post))
            .route(web::head().to(not_allowed)),
    )
    .service(
        web::resource("/register")
            .route(web::get().to(register_get))
            .route(web::post().to(register_post))
            .route(web::head().to(not_allowed)),
    );
}
