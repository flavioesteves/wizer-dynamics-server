use mongodb::Client;
use server_wizer::configuration::get_configuration;
use server_wizer::middleware::jwt_config::Config;
use server_wizer::startup::run;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
}

pub struct TestApp {
    pub address: String,
}

async fn spawn_app() -> TestApp {
    let config = Config::init();
    let c = get_configuration().expect("Failed to read configuration");

    let mongodb_uri = format!(
        "{}://{}:{}",
        c.database.model, c.database.host, c.database.port,
    );
    let db_client = Client::with_uri_str(mongodb_uri)
        .await
        .expect("Failed to connect");

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    println!("listener: {:?}", listener);
    println!("port: {:?}", port);

    let address = format!("http://127.0.0.1:{}", port);
    println!("address: {:?}", address);

    let server = run(listener, config, db_client).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp { address }
}
