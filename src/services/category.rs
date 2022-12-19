use std::fmt::format;

use actix_web::{http::StatusCode, web, Result};

use crate::DbPool;
use crate::{
    libs::{constants, error::ServiceError},
    models::{
        category::{Category, CategoryDTO},
        filters::CategoryFilter,
        response::Page,
    },
    schema::categories,
};

pub fn find_all(pool: &web::Data<DbPool>) -> Result<Vec<Category>, ServiceError> {
    match Category::find_all(&mut pool.get().unwrap()) {
        Ok(user) => Ok(user),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn filter(
    filter: CategoryFilter,
    pool: &web::Data<DbPool>,
) -> Result<Page<Category>, ServiceError> {
    match Category::filter(filter, &mut pool.get().unwrap()) {
        Ok(categories) => Ok(categories),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn find_by_id(id: i32, pool: &web::Data<DbPool>) -> Result<Category, ServiceError> {
    match Category::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(categories) => Ok(categories),
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Category with id {} not found", id),
        )),
    }
}

pub fn insert(new_category: CategoryDTO, pool: &web::Data<DbPool>) -> Result<(), ServiceError> {
    match Category::insert(new_category, &mut pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
        )),
    }
}

pub fn delete(id: i32, pool: &web::Data<DbPool>) -> Result<(), ServiceError> {
    match Category::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(_) => match Category::delete(id, &mut pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_CAN_NOT_DELETE_DATA.to_string(),
            )),
        },
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Category with id {} not found", id),
        )),
    }
}

pub fn update(
    id: i32,
    updated_category: CategoryDTO,
    pool: &web::Data<DbPool>,
) -> Result<(), ServiceError> {
    match Category::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(_) => match Category::update(id, updated_category, &mut pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_CAN_NOT_UPDATE_DATA.to_string(),
            )),
        },
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Category with id {} not found", id),
        )),
    }
}
