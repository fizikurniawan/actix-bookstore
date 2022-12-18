use crate::DbPool;

use actix_web::{get, web, Error, HttpResponse};
use diesel::prelude::*;

use crate::models::book::Book;

type DbError = Box<dyn std::error::Error + Send + Sync>;


#[get("/books")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let books = web::block(move || {
    let conn = &mut pool.get().expect("couldn't get db connection from pool");
    find_all(conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(books))
}



fn find_all(conn: &mut PgConnection) -> Result<Vec<Book>, DbError> {
  use crate::schema::books::dsl::*;

  let items = books.load::<Book>(conn)?;
  Ok(items)
}
