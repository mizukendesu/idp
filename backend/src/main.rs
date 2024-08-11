use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_web::web::scope;
use actix_web::HttpResponse;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::env;

mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use crate::models::book::Book;

async fn books_index(pool: web::Data<DbPool>) -> Result<HttpResponse, actix_web::Error> {
    use crate::schema::books::dsl::*;
    let mut conn = pool.get().map_err(|e| {
        log::error!("Couldn't get DB connection from pool: {:?}", e);
        actix_web::error::ErrorInternalServerError("Couldn't get DB connection from pool")
    })?;

    let results = books
        .limit(5)
        .load::<Book>(&mut conn)
        .map_err(|e| {
            log::error!("Error loading books: {:?}", e);
            actix_web::error::ErrorInternalServerError("Error loading books")
        })?;

    Ok(HttpResponse::Ok().json(results))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    if dotenv::dotenv().is_err() {
        log::warn!("⚠️  .env file not found, relying on environment variables");
    }

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(
                scope("/api")
                    .service(
                        scope("/v1")
                            .route("/books", web::get().to(books_index))
                    )
            )
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
