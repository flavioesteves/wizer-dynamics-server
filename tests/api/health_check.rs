use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check_status_200() {
    // Arrange
    let app = spawn_app().await;
    // Act
    let response = app
        .api_client
        .get(&format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), 200);
}
