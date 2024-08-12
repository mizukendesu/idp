use crate::domain::book::NewBook;
use crate::services::book_service::BookService;
use actix_web::{web, Error, HttpResponse, Result};

pub async fn list(service: web::Data<BookService>) -> Result<HttpResponse, Error> {
    let books = service.list().map_err(|e| {
        log::error!("Error loading books: {:?}", e);
        actix_web::error::ErrorInternalServerError("Error loading books")
    })?;

    Ok(HttpResponse::Ok().json(books))
}

pub async fn create(
    service: web::Data<BookService>,
    new_book: web::Json<NewBook>,
) -> Result<HttpResponse, Error> {
    let book = service.create(new_book.into_inner()).map_err(|e| {
        log::error!("Error creating book: {:?}", e);
        actix_web::error::ErrorInternalServerError("Error creating book")
    })?;

    Ok(HttpResponse::Created().json(book))
}
