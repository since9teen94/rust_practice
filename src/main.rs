use actix_files as fs;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, web, App, HttpServer};
use dotenvy::dotenv;
use std::env;
use web_app::not_found;
use web_app::{routes::home, routes::index};

///Be sure to set DATABASE_URL, PORT, and RUST_LOG .env variables to run the binary

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| String::from("3000"))
        .parse()
        .expect("Error parsing PORT variable: ");

    HttpServer::new(|| {
        App::new()
            .wrap(IdentityMiddleware::default())
            .wrap(Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    .build(),
            )
            .configure(index)
            .configure(home::index)
            .service(fs::Files::new("/static", "./static"))
            .default_service(web::to(not_found))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
