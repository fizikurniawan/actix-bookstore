use serde::{Deserialize, Serialize};

use crate::schema::categories;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = categories)]
pub struct NewCategory<'a> {
    pub name: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name = categories)]
pub struct CategoryDTO {
    pub name: String,
}
