use crate::domain::book::{Book, NewBook};
use crate::schema::books::dsl::*;
use diesel::prelude::*;
use diesel::PgConnection;

pub struct BookRepository<'a> {
    conn: &'a mut PgConnection,
}

impl<'a> BookRepository<'a> {
    pub fn new(conn: &'a mut PgConnection) -> Self {
        BookRepository { conn }
    }

    pub fn list(&mut self) -> QueryResult<Vec<Book>> {
        books.load::<Book>(self.conn)
    }

    pub fn create(&mut self, new_book: &NewBook) -> QueryResult<Book> {
        diesel::insert_into(books)
            .values((title.eq(&new_book.title), author.eq(&new_book.author)))
            .get_result(self.conn)
    }
}
