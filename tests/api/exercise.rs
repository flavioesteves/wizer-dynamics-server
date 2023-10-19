use crate::helpers::spawn_app;

//TODO remove the dependency of the database
// replace by a mockup DB

struct ExerciseData {
    id: String,
}

#[tokio::test]
async fn get_all_exercises_status_200() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app
        .api_client
        .get(&format!("{}/exercise", &app.address))
        .send()
        .await
        .expect("Failed to execute the request.");

    // Assert
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn crud_exercise_status_200() {
    // Arrange
    let app = spawn_app().await;

    //Act
    let response = app
        .api_client
        .post(&format!("{}/exercise", &app.address))
        .send()
        .await
        .expect("Failed to add a new exercise POST request.");

    println!("Response {:?}", response);
}
