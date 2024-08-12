use crate::domain::book::{Book, NewBook};
use crate::repositories::book_repository::BookRepository;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub struct BookService {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl BookService {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        BookService { pool }
    }

    pub fn list(&self) -> Result<Vec<Book>, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        let mut repo = BookRepository::new(&mut conn);
        repo.list()
    }

    pub fn create(&self, new_book: NewBook) -> Result<Book, diesel::result::Error> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");
        let mut repo = BookRepository::new(&mut conn);
        repo.create(&new_book)
    }
}
