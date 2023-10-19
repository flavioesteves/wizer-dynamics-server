use crate::helpers::spawn_app;

#[tokio::test]
async fn get_all_users_status_200() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app
        .api_client
        .get(&format!("{}/user/get_all_users", &app.address))
        .send()
        .await
        .expect("Failed to execute the request.");

    // Assert
    assert_eq!(response.status(), 200);
}
