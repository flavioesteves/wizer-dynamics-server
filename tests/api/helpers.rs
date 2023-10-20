use mongodb::Client;
use server_wizer::configuration::{get_configuration, DatabaseSettings};
use server_wizer::middleware::jwt_config::Config;
use server_wizer::startup::run;
use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
    pub api_client: reqwest::Client,
}

pub async fn spawn_app() -> TestApp {
    let jwt_config = Config::init();
    let app_config = get_configuration().expect("Failed to read configuration.");
    let db_client = create_mongodb_client(&app_config.database).await;

    println!("APP Config: {:?}", app_config);

    let test_listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let test_port = test_listener.local_addr().unwrap().port();
    let test_address = format!("http://127.0.0.1:{}", test_port);
    println!("Mock address: {:?}", test_address);

    let mock_server = run(test_listener, jwt_config, db_client)
        .expect("Failed to create the mock server wih the mock parameters");

    let _ = tokio::spawn(mock_server);
    let client = reqwest::Client::builder().build().unwrap();

    println!("Client: {:?}", client);

    TestApp {
        address: test_address,
        api_client: client,
    }
}

async fn create_mongodb_client(config: &DatabaseSettings) -> Client {
    let mongodb_uri = format!("{}://{}:{}", config.model, config.host, config.port,);
    Client::with_uri_str(&mongodb_uri)
        .await
        .expect("Failed to connect to database")
}
