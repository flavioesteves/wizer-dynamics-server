use crate::helpers::spawn_app;

//TODO remove the dependency of the database
// replace by a mockup DB

#[tokio::test]
async fn get_all_exercises_status_200() {
    // Arrange
    let app = spawn_app().await;

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/exercise", &app.address))
        .send()
        .await
        .expect("Failed to execute the request.");

    // Assert
    assert_eq!(response.status(), 200);
}
