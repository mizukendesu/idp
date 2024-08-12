use actix_web::{middleware::Logger, web, App, HttpServer};
use std::env;

mod controllers;
mod domain;
mod infrastructure;
mod repositories;
mod schema;
mod services;

use crate::controllers::book_controller;
use crate::infrastructure::db::establish_connection;
use crate::services::book_service::BookService;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    if dotenv::dotenv().is_err() {
        log::warn!("⚠️  .env file not found, relying on environment variables");
    }

    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = establish_connection(&database_url);
    let book_service = web::Data::new(BookService::new(pool.clone()));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(book_service.clone())
            .service(
                web::scope("/api/v1")
                    .route("/books", web::get().to(book_controller::list))
                    .route("/books", web::post().to(book_controller::create)),
            )
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
