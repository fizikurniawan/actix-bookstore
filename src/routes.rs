use crate::handlers::{book, user};
use actix_web::{http::StatusCode, web, HttpResponse, Result};

async fn not_found() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type("text/html; charset=utf-8")
        .body("<h1>Error 404</h1>"))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("api")
            .service(web::scope("books").service(book::index))
            .service(
                web::scope("users")
                    .service(user::find_all)
                    .service(user::filter)
                    .service(user::show)
                    .service(user::insert)
                    .service(user::update),
            ),
    )
    .default_service(web::route().to(not_found));
}
