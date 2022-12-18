use actix_web::{HttpRequest, Responder, http, web};

fn foo(req: HttpRequest) -> impl Responder {
    (
        web::Json(Status {
            value: val,
            isOn: true,
        }),
        http::StatusCode::CONFLICT,
    )
}