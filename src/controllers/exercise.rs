use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use mongodb::bson::oid::ObjectId;

use crate::database::exercise_db;
use crate::middleware::jwt_model::AppState;
use crate::models::exercise::Exercise;

pub async fn get_all_exercises(data: Data<AppState>) -> HttpResponse {
    let exercises = exercise_db::get_all_exercises(data.client.clone())
        .await
        .expect("CT Exercise: Error failed to retrieve from DB");

    HttpResponse::Ok().json(exercises)
}

pub async fn add_exercise(data: Data<AppState>, req: Json<Exercise>) -> HttpResponse {
    let exercise = Exercise::new(
        req.name.clone(),
        req.steps.clone(),
        req.video.clone(),
        req.photo.clone(),
    );
    let insert;
    {
        match exercise_db::post_exercise(data.client.clone(), exercise.clone()).await {
            Ok(result) => insert = result,
            Err(err) => {
                eprintln!("CT Exercise: Error to post {:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        };
    }
    HttpResponse::Ok().json(insert)
}

pub async fn get_exercise_by_id(data: Data<AppState>, _id: Path<String>) -> HttpResponse {
    let exercise = exercise_db::get_exercise_by_id(data.client.clone(), _id.clone())
        .await
        .expect("CT Exercise: Error failed to retrieve the exercise with _id");

    HttpResponse::Ok().json(&exercise)
}

pub async fn update_exercise_by_id(
    data: Data<AppState>,
    _id: Path<String>,
    req: Json<Exercise>,
) -> HttpResponse {
    let id = _id.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid ID");
    };

    let exercise_data = Exercise {
        _id: Some(ObjectId::parse_str(&id).unwrap()),
        name: req.name.clone(),
        steps: req.steps.clone(),
        video: req.video.clone(),
        photo: req.photo.clone(),
    };

    let updated_exercise = exercise_db::update_exercise(data.client.clone(), exercise_data)
        .await
        .expect("CT Exercise: Error failed to update the exercise");

    HttpResponse::Ok().json(&updated_exercise)
}

pub async fn delete_exercise_by_id(data: Data<AppState>, _id: Path<String>) -> HttpResponse {
    let exercise = exercise_db::delete_exercise_by_id(data.client.clone(), _id.clone())
        .await
        .expect("CT Exercise: Error failed to delete the exercise");

    HttpResponse::Ok().json(&exercise)
}
