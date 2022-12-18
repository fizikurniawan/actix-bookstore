use crate::{
    libs::{constants, error::ServiceError},
    models::{filters::UserFilter, response::Page, user::{User, UserDTO}},
    DbPool,
};
use actix_web::{http::StatusCode, web};

pub fn find_all(pool: &web::Data<DbPool>) -> Result<Vec<User>, ServiceError> {
    match User::find_all(&mut pool.get().unwrap()) {
        Ok(user) => Ok(user),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn filter(filter: UserFilter, pool: &web::Data<DbPool>) -> Result<Page<User>, ServiceError> {
    match User::filter(filter, &mut pool.get().unwrap()) {
        Ok(users) => Ok(users),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_FETCH_DATA.to_string(),
        )),
    }
}

pub fn find_by_id(id: i32, pool: &web::Data<DbPool>) -> Result<User, ServiceError> {
    match User::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(users) => Ok(users),
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Person with id {} not found", id),
        )),
    }
}

pub fn insert(new_user: UserDTO, pool: &web::Data<DbPool>) -> Result<(), ServiceError> {
    match User::insert(new_user, &mut pool.get().unwrap()) {
        Ok(_) => Ok(()),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
        )),
    }
}

pub fn update(
    id: i32,
    updated_person: UserDTO,
    pool: &web::Data<DbPool>,
) -> Result<(), ServiceError> {
    match User::find_by_id(id, &mut pool.get().unwrap()) {
        Ok(_) => match User::update(id, updated_person, &mut pool.get().unwrap()) {
            Ok(_) => Ok(()),
            Err(_) => Err(ServiceError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                constants::MESSAGE_CAN_NOT_UPDATE_DATA.to_string(),
            )),
        },
        Err(_) => Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("User with id {} not found", id),
        )),
    }
}