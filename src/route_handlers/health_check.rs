use actix_web::{web, web::ServiceConfig, Error, HttpResponse};

pub fn config_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/health_check").service(web::resource("").route(web::get().to(health_check))),
    );
}

pub async fn health_check() -> Result<HttpResponse, Error> {
    let message = "The server is running";

    Ok(HttpResponse::Ok().json(message))
}
