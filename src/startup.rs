use crate::middleware::{jwt_config::Config, jwt_model::AppState};

use actix_cors::Cors;
use actix_web::{dev::Server, middleware::Logger, web, App, HttpServer};
use mongodb::Client;
use std::net::TcpListener;

use crate::route_handlers::{exercise, health_check, jwt, training_plan, user};

pub fn run(
    listener: TcpListener,
    config: Config,
    db_client: Client,
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new({
                AppState {
                    env: config.clone(),
                    client: db_client.clone(),
                }
            }))
            .configure(user::config_routes)
            .configure(exercise::config_routes)
            .configure(training_plan::config_routes)
            .configure(jwt::config_routes)
            .configure(health_check::config_routes)
            .wrap(
                //TODO remove this devMode
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
                    .allow_any_header(),
            )
            .wrap(Logger::default())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
