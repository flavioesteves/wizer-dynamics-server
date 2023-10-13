use mongodb::Client;
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
    //TODO create a database pool for tests
    //For now is using the same logic of prod
    let config = Config::init();
    //TODO Replace this logic from here
    let mongodb_uri = "mongodb://localhost:27017";
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
