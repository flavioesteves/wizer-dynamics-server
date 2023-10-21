use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_status_200() {
    // Arrange
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    // Act

    println!("Address:{:?}", app.address);
    let response = client
        .get(&format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
}
