use actix_web::{web, web::ServiceConfig};

use crate::middleware::jwt_controller;

#[tracing::instrument(name = "Route Handler: JWT", skip(cfg))]
pub fn config_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/auth/login")
            .service(web::resource("").route(web::post().to(jwt_controller::login))),
    )
    .service(
        web::scope("/auth/register")
            .service(web::resource("").route(web::post().to(jwt_controller::register))),
    );
}
