use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, Error, HttpResponse, HttpServer};

use mongodb::Client;

mod controllers;
mod database;
mod middleware;
mod models;
mod route_handlers;

use dotenv::dotenv;
use middleware::{jwt_config::Config, jwt_model::AppState};
use route_handlers::{exercise, jwt, training_plan, user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let is_cloud = false;
    let config = Config::init();
    let mut uri =
        std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".into());

    if is_cloud {
        uri = std::env::var("MONGODB_URI_CLOUD")
            .unwrap_or_else(|_| "mongodb://localhost:270217".into());
    }
    let client = Client::with_uri_str(uri).await.expect("failed to connect");

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
            .wrap(
                //TODO remove this devMode
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
                    .allow_any_header(),
            )
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
