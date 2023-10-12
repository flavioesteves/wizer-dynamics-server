use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};

use mongodb::Client;
use server_wizer::configuration::get_configuration;
use server_wizer::middleware::{jwt_config::Config, jwt_model::AppState};
use server_wizer::route_handlers::{exercise, health_check, jwt, training_plan, user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::init();
    let configuration = get_configuration().expect("Failed to read configuration");

    //TODO Replace this logic from here
    let mongodb_uri = "mongodb://localhost:27017";

    let client = Client::with_uri_str(mongodb_uri)
        .await
        .expect("failed to connect");

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                env: config.clone(),
                client: client.clone(),
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
    .bind((
        configuration.application.host,
        configuration.application.port,
    ))?
    .run()
    .await
}
