use std::net::TcpListener;

use mongodb::Client;
use server_wizer::configuration::get_configuration;
use server_wizer::middleware::jwt_config::Config;
use server_wizer::startup::run;

//#[tokio::main]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::init();
    let configuration = get_configuration().expect("Failed to read configuration");

    //TODO Replace this logic from here
    let mongodb_uri = "mongodb://localhost:27017";

    let db_client = Client::with_uri_str(mongodb_uri)
        .await
        .expect("Failed to connect");

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    println!("Main address {}", address);
    let listener = TcpListener::bind(address)?;
    run(listener, config, db_client)?.await?;
    Ok(())
}
