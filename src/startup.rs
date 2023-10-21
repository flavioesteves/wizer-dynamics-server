use crate::configuration::{DatabaseSettings, Settings};
use crate::middleware::{jwt_config::Config, jwt_model::AppState};

use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{dev::Server, App, HttpServer};
use mongodb::Client;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::route_handlers::{exercise, health_check, jwt, training_plan, user};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let jwt_config = Config::init();
        let db_client = get_db_client(&configuration.database).await;
        let app_address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port,
        );

        println!("Main address {}", app_address);
        let app_listener = TcpListener::bind(app_address)?;
        let port = app_listener.local_addr().unwrap().port();
        let server = run(app_listener, jwt_config, db_client)?;

        Ok(Self { server, port })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

async fn get_db_client(db_settings: &DatabaseSettings) -> Client {
    let mongodb_uri = format!(
        "{}://{}:{}",
        db_settings.model, db_settings.host, db_settings.port,
    );

    Client::with_uri_str(mongodb_uri)
        .await
        .expect("Failed to connect to the database.")
}

pub fn run(
    listener: TcpListener,
    config: Config,
    db_client: Client,
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new({
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
            .wrap(TracingLogger::default())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
