use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = users)]
pub struct UserDTO {
    pub name: String,
    pub email: String,
}
