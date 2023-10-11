use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use mongodb::bson::oid::ObjectId;

use crate::database::training_plan_db;
use crate::middleware::jwt_model::AppState;
use crate::models::training_plan::TrainingPlan;

pub async fn get_all_trainings(data: Data<AppState>) -> HttpResponse {
    let exercises = training_plan_db::get_all_trainings(data.client.clone())
        .await
        .expect("CT Training: Error failed to retrieve from DB");

    HttpResponse::Ok().json(exercises)
}

pub async fn add_training(data: Data<AppState>, req: Json<TrainingPlan>) -> HttpResponse {
    let exercise = TrainingPlan::new(
        req.day.clone(),
        req.theme.clone(),
        req.estimated_time.clone(),
        req.schedule_days.clone(),
    );
    let insert = training_plan_db::post_training(data.client.clone(), exercise)
        .await
        .expect("CT Training: Error to post");

    HttpResponse::Ok().json(insert)
}

pub async fn get_training_by_id(data: Data<AppState>, _id: Path<String>) -> HttpResponse {
    println!("exercise _id: {}", _id);
    let exercise = training_plan_db::get_training_by_id(data.client.clone(), _id.clone())
        .await
        .expect("CT Training: Error failed to retrieve the exercise with _id");

    HttpResponse::Ok().json(exercise)
}

pub async fn update_training_by_id(
    data: Data<AppState>,
    _id: Path<String>,
    req: Json<TrainingPlan>,
) -> HttpResponse {
    let id = _id.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid ID");
    };

    let training_data = TrainingPlan {
        _id: Some(ObjectId::parse_str(&id).unwrap()),
        day: req.day.clone(),
        theme: req.theme.clone(),
        estimated_time: req.estimated_time.clone(),
        schedule_days: req.schedule_days.clone(),
    };

    let updated_training = training_plan_db::update_training(data.client.clone(), training_data)
        .await
        .expect("CT Training: Error failed to update the training");
    return HttpResponse::Ok().json(updated_training);
}

pub async fn delete_training_by_id(data: Data<AppState>, _id: Path<String>) -> HttpResponse {
    let exercise = training_plan_db::delete_training_by_id(data.client.clone(), _id.clone())
        .await
        .expect("CT Training: Error failed to delete the user");

    HttpResponse::Ok().json(exercise)
}
