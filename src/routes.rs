use actix_web::{
    web::{self, Form, Json},
    Either, Responder,
};
use actix_web_lab::web::Redirect;
use tera::Context;
use validator::Validate;
use web_app::{
    models::{UserLogin, UserRegistration},
    not_allowed, register, render,
};

type RegisterNewUser = Either<Json<UserRegistration>, Form<UserRegistration>>;
type LoginUser = Either<Json<UserLogin>, Form<UserLogin>>;

async fn login_get() -> impl Responder {
    let mut context = Context::new();
    context.insert("title", "Login");
    render("log_reg.html", context)
}

async fn login_post(login_data: LoginUser) -> impl Responder {
    let login = login_data.into_inner();
    if let Err(e) = login
        .validate()
        .map_err(|e| serde_json::to_string(&e).unwrap())
    {
        return e;
    };
    String::from("User logged in successfully")
}

async fn register_get() -> impl Responder {
    let mut context = Context::new();
    context.insert("title", "Register");
    render("log_reg.html", context)
}

async fn register_post(registration_data: RegisterNewUser) -> impl Responder {
    let registration_values = registration_data.into_inner();
    if let Err(e) = registration_values
        .validate()
        .map_err(|e| serde_json::to_string(&e).unwrap())
    {
        return e;
    };
    //TODO return as server errs
    if let Err(e) = register(registration_values) {
        return serde_json::to_string(&e).unwrap();
    };
    String::from("User successfully registered")
}

pub fn index(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(|| async { Redirect::new("/", "/login") }))
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
