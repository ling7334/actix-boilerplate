use actix_web::{get, guard, middleware, web, App, HttpResponse, HttpServer, Responder};

/// 404 handler
async fn p404() -> impl Responder {
    format!("404")
}

#[get("/")]
async fn index() -> impl Responder {
    format!("ok")
}

#[actix_web::main]
pub async fn run(addr: String) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register endpoints
            .service(index)
            // default
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(p404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })
    .bind(addr)?
    .run()
    .await
}
