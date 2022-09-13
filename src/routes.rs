use actix_identity::Identity;
use actix_web::{
    http::{self, header, header::HeaderValue, StatusCode},
    web::{self, Form, Json},
    Either, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use actix_web_lab::web::Redirect;
//use log::debug;
use serde_json::json;
use tera::Context;
use uuid::Uuid;
use validator::Validate;
use web_app::{
    forms::LogRegForm,
    models::{UserLogin, UserRegistration},
    not_allowed, register, render, response, /* HTML,*/ JSON,
};
//TODO more tests?
//TODO homepage frontend

type RegisterNewUser = Either<Json<UserRegistration>, Form<UserRegistration>>;
type LoginUser = Either<Json<UserLogin>, Form<UserLogin>>;

async fn login_get() -> impl Responder {
    let login_form = LogRegForm::new("Log In", "/login");
    let context = Context::from_serialize(login_form).unwrap();
    render("log_reg.html", context)
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
    let register_form = LogRegForm::new("Register", "/register");
    let context = Context::from_serialize(register_form).unwrap();
    render("log_reg.html", context)
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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_identity::IdentityMiddleware;
    use actix_session::{storage::CookieSessionStore, SessionMiddleware};
    use actix_web::body::MessageBody;
    use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
    use actix_web::{cookie::Key, middleware::Logger};
    use actix_web::{test, App, Error};
    use std::collections::HashMap;
    fn start_app() -> App<
        impl ServiceFactory<
            ServiceRequest,
            Response = ServiceResponse<impl MessageBody>,
            Config = (),
            InitError = (),
            Error = Error,
        >,
    > {
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .build(),
            )
            .service(
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
    }

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(start_app()).await;
        let request = test::TestRequest::get().uri("/").to_request();
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), 307);
    }

    #[actix_web::test]
    async fn test_index_post_no_data() {
        let app = test::init_service(start_app()).await;
        let request = test::TestRequest::post().uri("/").to_request();
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), 405);
    }

    #[actix_web::test]
    async fn test_login_get() {
        let app = test::init_service(start_app()).await;
        let request = test::TestRequest::get().uri("/login").to_request();
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), 200);
    }

    #[actix_web::test]
    async fn test_login_post_no_data() {
        let app = test::init_service(start_app()).await;
        let request = test::TestRequest::post().uri("/login").to_request();
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), 400);
    }

    #[actix_web::test]
    async fn test_invalid_route() {
        let app = test::init_service(start_app()).await;
        let request = test::TestRequest::post()
            .uri("/this_does_not_exist")
            .to_request();
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), 404);
    }

    #[actix_web::test]
    async fn correct_login() {
        let app = test::init_service(start_app()).await;
        let data = json!({
            "email" : "frodo@theshire.com",
            "password" : "Password1!",
        });
        let request = test::TestRequest::post()
            .uri("/login")
            .set_json(data)
            .to_request();
        let response = test::call_service(&app, request).await;
        //assert_eq!(response.status(), 200);
        assert_eq!(response.status(), 303);
    }

    #[actix_web::test]
    async fn incorrect_and_valid_password_login() {
        let app = test::init_service(start_app()).await;
        let data = json!({
            "email" : "frodo@theshire.com",
            "password" : "Password12!",
        });
        let request = test::TestRequest::post()
            .uri("/login")
            .set_json(data)
            .to_request();
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), 400);
    }

    #[actix_web::test]
    async fn incorrect_and_valid_email_login() {
        let app = test::init_service(start_app()).await;
        let data = json!({
            "email" : "frodo@theshire",
            "password" : "Password1!",
        });
        let request = test::TestRequest::post()
            .uri("/login")
            .set_json(data)
            .to_request();
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), 400);
    }

    #[actix_web::test]
    async fn invalid_email_login() {
        let app = test::init_service(start_app()).await;
        let data = json!({
            "email" : "frodo",
            "password" : "Password1!",
        });
        let request = test::TestRequest::post()
            .uri("/login")
            .set_json(data)
            .to_request();
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), 400);
    }

    #[actix_web::test]
    async fn invalid_login_email_response_body() {
        let app = test::init_service(start_app()).await;
        let data = json!({
            "email" : "frodo",
            "password" :"Password1!",
        });
        let request = test::TestRequest::post()
            .uri("/login")
            .set_json(data)
            .to_request();
        let response = test::call_and_read_body(&app, request).await;
        let left: HashMap<String, serde_json::Value> = serde_json::from_slice(
            b"{\"__all__\":[{\"code\":\"invalid\",\"message\":\"Invalid Credentials\",\"params\":{}}],\"email\":[{\"code\":\"email\",\"message\":null,\"params\":{\"value\":\"frodo\"}}]}").unwrap();
        let right: HashMap<String, serde_json::Value> = serde_json::from_slice(&response).unwrap();
        assert_eq!(left, right)
    }

    #[actix_web::test]
    async fn invalid_login_password_response_body() {
        let app = test::init_service(start_app()).await;
        let data = json!({
            "email" : "frodo@theshire.com",
            "password" :"Password1",
        });
        let request = test::TestRequest::post()
            .uri("/login")
            .set_json(data)
            .to_request();
        let response = test::call_and_read_body(&app, request).await;
        let left: HashMap<String, serde_json::Value> = serde_json::from_slice(
            b"{\"__all__\":[{\"code\":\"invalid\",\"message\":\"Invalid Credentials\",\"params\":{}}]}",
        )
        .unwrap();
        let right: HashMap<String, serde_json::Value> = serde_json::from_slice(&response).unwrap();
        assert_eq!(left, right)
    }
}
