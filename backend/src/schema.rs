// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        #[max_length = 100]
        title -> Varchar,
        #[max_length = 100]
        author -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
