use crate::forms::LogRegForm;
use crate::models::{UserLogin, UserRegistration};
use crate::{not_allowed, register, render, response, /* HTML,*/ JSON};
use actix_identity::Identity;
use actix_web::{
    http::{self, header, header::HeaderValue, StatusCode},
    web::{self, Form, Json},
    Either, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use actix_web_lab::web::Redirect;
use serde_json::json;
use tera::Context;
use uuid::Uuid;
use validator::Validate;
//TODO homepage frontend, routes

type RegisterNewUser = Either<Json<UserRegistration>, Form<UserRegistration>>;
type LoginUser = Either<Json<UserLogin>, Form<UserLogin>>;

async fn login_get() -> impl Responder {
    let login_form = LogRegForm::new("Log In", "/login", "POSt");
    let context = Context::from_serialize(login_form).unwrap();
    render("logReg.html", context)
}

async fn login_post(
    req: HttpRequest,
    login_data: LoginUser,
    user: Option<Identity>,
) -> impl Responder {
    if user.is_some() {
        //mimic 2xx/4xx client-side redirects
        let mut response = response(303, *JSON, None);
        response
            .headers_mut()
            .append(header::LOCATION, HeaderValue::from_static("/home"));
        response
    } else {
        let login = login_data.into_inner();
        if let Err(e) = login.validate() {
            let body = serde_json::to_string(&e).unwrap();
            return response(400, *JSON, Some(body));
        };
        let id = Uuid::new_v4();
        Identity::login(&req.extensions(), id.to_string()).unwrap();
        let body = json!({ "message": "User Logged In Successfully" }).to_string();
        //mimic 2xx/4xx client-side redirects
        let mut response = response(303, *JSON, Some(body));
        response
            .headers_mut()
            .append(header::LOCATION, HeaderValue::from_static("/home"));
        response
    }
}

async fn register_get() -> impl Responder {
    let register_form = LogRegForm::new("Register", "/register", "POST");
    let context = Context::from_serialize(register_form).unwrap();
    render("logReg.html", context)
}

async fn register_post(
    req: HttpRequest,
    registration_data: RegisterNewUser,
    user: Option<Identity>,
) -> impl Responder {
    if user.is_some() {
        //mimic 2xx/4xx client-side redirects
        let mut response = response(303, *JSON, None);
        response
            .headers_mut()
            .append(header::LOCATION, HeaderValue::from_static("/home"));
        return response;
    };
    let registration_values = registration_data.into_inner();
    if let Err(e) = registration_values.validate() {
        let e = serde_json::to_string(&e).unwrap();
        return response(400, *JSON, Some(e));
    };
    if let Err(e) = register(registration_values).await {
        let e = serde_json::to_string(&e).unwrap();
        return response(400, *JSON, Some(e));
    };
    let id = Uuid::new_v4();
    Identity::login(&req.extensions(), id.to_string()).unwrap();
    let body = json!({
            "message": "User Registered Successfully"
    })
    .to_string();
    //mimic 2xx/4xx client-side redirects
    let mut response = response(303, *JSON, Some(body));
    response
        .headers_mut()
        .append(header::LOCATION, HeaderValue::from_static("/home"));
    response
}

async fn logout(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        user.logout();
    };
    HttpResponse::build(StatusCode::from_u16(302).unwrap())
        .append_header((http::header::LOCATION, "/login"))
        .finish()
}

async fn index_get() -> impl Responder {
    Redirect::new("/", "/login")
}

async fn home_get(user: Option<Identity>) -> impl Responder {
    if user.is_none() {
        //mimic 2xx/4xx client-side redirects
        let mut response = response(303, *JSON, None);
        response
            .headers_mut()
            .append(header::LOCATION, HeaderValue::from_static("/login"));
        return response;
    };
    let mut context = Context::new();
    context.insert("title", "Home");
    render("home.html", context)
}

pub fn index(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(index_get))
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
    )
    .service(
        web::resource("/logout")
            .route(web::get().to(logout))
            .route(web::post().to(logout))
            .route(web::to(not_allowed)),
    )
    .service(
        web::resource("/home")
            .route(web::get().to(home_get))
            .route(web::to(not_allowed)),
    );
}

//cfg test from index routes if necessary
