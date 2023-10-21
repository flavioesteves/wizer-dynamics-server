use crate::helpers::spawn_app;

//TODO remove the dependency of the database
// replace by a mockup DB

#[tokio::test]
async fn get_all_trainings_status_200() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let address = format!("{}/training-plan", &app.address);
    // Act
    let response = client
        .get(&address)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(response.status(), 200);
}
