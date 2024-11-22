use actix_web::{dev::ServiceFactory, middleware, web, App, Error, HttpRequest, HttpResponse};
use features::systeminfo::handlers;

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
        .route(
            "/fetch",
            web::get().to(|req: HttpRequest| async move {
                let params =
                    web::Query::<Vec<(String, String)>>::from_query(req.query_string()).unwrap();
                let res = handlers::fetchservice_handler(params.into_inner());
                HttpResponse::Ok().body(serde_json::to_string(&res).unwrap())
            }),
        )
        .route("/echo", web::get().to(ws_utils::echo))
}
