use diesel::prelude::*;
use crate::schema::books;
use serde::Serialize;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = books)]
pub struct NewBook<'a> {
    pub title: &'a str,
    pub author: &'a str,
}
