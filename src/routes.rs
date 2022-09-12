use actix_session::Session;
use actix_web::{
    web::{self, Form, Json},
    Either, Responder,
};
use actix_web_lab::web::Redirect;
//use std::borrow::Cow;
use tera::Context;
use uuid::Uuid;
use validator::Validate;
use web_app::{
    bad_req,
    forms::LogRegForm,
    good_req,
    models::{UserLogin, UserRegistration},
    not_allowed, register, render, HTML, JSON,
};

type RegisterNewUser = Either<Json<UserRegistration>, Form<UserRegistration>>;
type LoginUser = Either<Json<UserLogin>, Form<UserLogin>>;

async fn login_get() -> impl Responder {
    let login_form = LogRegForm::new("Log In", "/login");
    let context = Context::from_serialize(login_form).unwrap();
    render("log_reg.html", context)
}

async fn login_post(login_data: LoginUser, session: Session) -> impl Responder {
    let login = login_data.into_inner();
    if let Err(e) = login.validate() {
        return bad_req(400, *JSON, e);
    };
    let id = Uuid::new_v4();
    //println!("{id}");
    session.insert("uuid", id.to_string()).unwrap();
    println!("this is the session data: {:?}", session.entries());
    good_req(200, *HTML, "User logged in successfully")
    //Cow('static, Redirect {
    //from: "/login",
    //to: "/home",
    //status_code: 300,
    //})
    //TODO flesh out cookies / auth
    //WIP redirect to home page
}

async fn register_get() -> impl Responder {
    let register_form = LogRegForm::new("Register", "/register");
    let context = Context::from_serialize(register_form).unwrap();
    render("log_reg.html", context)
}

async fn register_post(registration_data: RegisterNewUser) -> impl Responder {
    let registration_values = registration_data.into_inner();
    if let Err(e) = registration_values.validate() {
        return bad_req(400, *JSON, e);
    };
    if let Err(e) = register(registration_values) {
        return bad_req(400, *JSON, e);
    }
    good_req(201, *HTML, "User successfully registered")
    //WIP redirect to home page
}

pub fn index(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(|| async { Redirect::new("/", "/login") }))
            .route(web::to(not_allowed)),
    )
    .service(
        web::resource("/login")
            .route(web::get().to(login_get))
            .route(web::post().to(login_post))
            .route(web::to(not_allowed)),
    )
    .service(
        web::resource("/register")
            .route(web::get().to(register_get))
            .route(web::post().to(register_post))
            .route(web::to(not_allowed)),
    );
}
