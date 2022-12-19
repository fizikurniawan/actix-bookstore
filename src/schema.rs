// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    books,
    categories,
    users,
);
