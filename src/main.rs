use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use std::env;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| String::from("3000"))
        .parse()
        .expect("Error parsing PORT variable: ");

    HttpServer::new(|| App::new().configure(routes::index))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
