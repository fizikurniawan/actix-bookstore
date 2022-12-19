use crate::libs::constants;
use crate::models::category::CategoryDTO;
use crate::models::filters::CategoryFilter;
use crate::models::response::ResponseBody;
use crate::services::category as category_service;
use crate::DbPool;
use actix_web::{delete, get, post, put, web, Error, HttpResponse, Result};

#[get("")]
pub async fn find_all(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    match category_service::find_all(&pool) {
        Ok(categories) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, categories)))
        }
        Err(err) => Ok(err.response()),
    }
}

#[get("{id}")]
pub async fn show(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    match category_service::find_by_id(id.into_inner(), &pool) {
        Ok(categories) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, categories)))
        }
        Err(err) => Ok(err.response()),
    }
}

#[get("filter")]
pub async fn filter(
    web::Query(filter): web::Query<CategoryFilter>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    match category_service::filter(filter, &pool) {
        Ok(categories) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, categories)))
        }
        Err(err) => Ok(err.response()),
    }
}

#[post("")]
pub async fn insert(
    new_category: web::Json<CategoryDTO>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    match category_service::insert(new_category.0, &pool) {
        Ok(()) => Ok(HttpResponse::Created()
            .json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

#[put("{id}")]
pub async fn update(
    id: web::Path<i32>,
    updated_category: web::Json<CategoryDTO>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    match category_service::update(id.into_inner(), updated_category.0, &pool) {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY)))
        }
        Err(err) => Ok(err.response()),
    }
}

#[delete("{id}")]
pub async fn delete(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    match category_service::delete(id.into_inner(), &pool) {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY)))
        }
        Err(err) => Ok(err.response()),
    }
}
