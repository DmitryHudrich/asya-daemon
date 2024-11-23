use actix_web::{dev::ServiceFactory, middleware, web, App, Error};

mod ws_utils;

pub fn route_all() -> App<
    impl ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = Error,
        InitError = (),
    >,
> {
    App::new()
        .wrap(middleware::DefaultHeaders::new().add(("Content-Type", "application/json")))
        .route("/hey", web::get().to(|| async { "bebra" }))
        .route("/sex", web::get().to(|| async { "не было" }))
        .route("/echo", web::get().to(ws_utils::echo))
}
