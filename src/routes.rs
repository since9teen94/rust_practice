use actix_web::{
    web::{self, Form, Json},
    Either, HttpResponse, Responder,
};
use diesel::{insert_into, prelude::*};
use log::{debug, error, info, warn};
use validator::Validate;
use web_app::establish_connection;
use web_app::models::*;

type RegisterNewUser = Either<Json<UserRegistration>, Form<UserRegistration>>;
type LoginUser = Either<Json<UserLogin>, Form<UserLogin>>;

pub async fn not_allowed() -> impl Responder {
    HttpResponse::MethodNotAllowed()
}

async fn index_get() -> impl Responder {
    "hello world!"
}

async fn login_get() -> impl Responder {
    "Hello login_get"
}

async fn login_post(login_data: LoginUser) -> impl Responder {
    use web_app::schema::users::dsl::*;

    let login = login_data.into_inner();
    debug!("Logging in the following user: {login:#?}");
    //guard clause
    if let Err(e) = login
        .validate()
        .map_err(|e| serde_json::to_string(&e).unwrap())
    {
        return e.to_string();
    };
    let conn = &mut establish_connection();
    //We have made assurances that this is okay...
    let db_password = users
        .select(password)
        .filter(email.eq(login.email))
        .first::<String>(conn)
        .unwrap();
    if let Err(e) = compare_password(&login.password, &db_password) {
        debug!("{e}");
        return format!("{e}");
    };
    debug!("{}", db_password);
    String::from("User logged in successfully")
}

async fn register_get() -> impl Responder {
    "Hello register_get"
}

async fn register_post(registration_data: RegisterNewUser) -> impl Responder {
    use web_app::schema::users::dsl::*;

    let registration = registration_data.into_inner();
    debug!("Registering the following user: {registration:?}");
    match registration
        .validate()
        .map_err(|e| serde_json::to_string(&e).unwrap())
    {
        Ok(_) => {
            let conn = &mut establish_connection();
            let new_user = register(registration);

            insert_into(users)
                .values(new_user)
                .execute(conn)
                .expect("Error registering new user: ");

            info!("User successfully registered");
            String::from("User successfully registered")
        }
        Err(e) => {
            error!("Error(s) encountered registering user: {e}");
            e.to_string()
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
