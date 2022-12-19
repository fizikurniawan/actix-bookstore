use crate::models::filters::UserFilter;
use crate::models::user::UserDTO;
use crate::{libs::constants, DbPool};

use actix_web::{get, post, put, web, Error, HttpResponse, Result, delete};

use crate::models::response::ResponseBody;
use crate::services::user as user_service;

#[get("")]
pub async fn find_all(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    match user_service::find_all(&pool) {
        Ok(people) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, people))),
        Err(err) => Ok(err.response()),
    }
}

#[get("{id}")]
pub async fn show(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    match user_service::find_by_id(id.into_inner(), &pool) {
        Ok(people) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, people))),
        Err(err) => Ok(err.response()),
    }
}

#[get("filter")]
pub async fn filter(
    web::Query(filter): web::Query<UserFilter>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    match user_service::filter(filter, &pool) {
        Ok(page) => Ok(HttpResponse::Ok().json(page)),
        Err(err) => Ok(err.response()),
    }
}

#[post("")]
pub async fn insert(new_user: web::Json<UserDTO>, pool: web::Data<DbPool>) -> Result<HttpResponse> {
    match user_service::insert(new_user.0, &pool) {
        Ok(()) => Ok(HttpResponse::Created()
            .json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY))),
        Err(err) => Ok(err.response()),
    }
}

#[put("{id}")]
pub async fn update(
    id: web::Path<i32>,
    updated_person: web::Json<UserDTO>,
    pool: web::Data<DbPool>,
) -> Result<HttpResponse> {
    match user_service::update(id.into_inner(), updated_person.0, &pool) {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY)))
        }
        Err(err) => Ok(err.response()),
    }
}

#[delete("{id}")]
pub async fn delete(id: web::Path<i32>, pool: web::Data<DbPool>) -> Result<HttpResponse> {
    match user_service::delete(id.into_inner(), &pool) {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, constants::EMPTY)))
        }
        Err(err) => Ok(err.response()),
    }
}

