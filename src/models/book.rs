use serde::{Deserialize, Serialize};

use crate::schema::books;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Book {
  pub id: i32,
  pub name: String,
  pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = books)]
pub struct NewBook<'a> {
  pub name: &'a str,
  pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookPayload {
  pub name: String,
}