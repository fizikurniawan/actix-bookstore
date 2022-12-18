use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;

mod handlers;
mod libs;
mod models;
mod repository;
mod routes;
mod schema;
mod services;

// use handlers::{book, user};

#[macro_use]
extern crate diesel;
extern crate env_logger;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(|| async { "BookStore REST API" }))
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
