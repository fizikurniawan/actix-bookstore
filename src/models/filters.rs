use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserFilter {
    pub name: Option<String>,
    pub email: Option<String>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
    pub page_num: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Deserialize)]
pub struct CategoryFilter {
    pub name: Option<String>,
    pub sort_by: Option<String>,
    pub sort_direction: Option<String>,
    pub page_num: Option<i64>,
    pub page_size: Option<i64>,
}
