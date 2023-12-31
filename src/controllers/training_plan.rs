use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use mongodb::bson::oid::ObjectId;

use crate::database::training_plan_db;
use crate::middleware::jwt_model::AppState;
use crate::models::training_plan::TrainingPlan;

#[tracing::instrument(name = "Controller: Get All Trainings")]
pub async fn get_all_trainings(data: Data<AppState>) -> HttpResponse {
    let trainings = match training_plan_db::get_all_trainings(data.client.clone()).await {
        Ok(trainings) => trainings,
        Err(err) => {
            eprintln!("CT Trainigs: {:?}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };
    HttpResponse::Ok().json(&trainings)
}

#[tracing::instrument(name = "Controller: Add Training")]
pub async fn add_training(data: Data<AppState>, req: Json<TrainingPlan>) -> HttpResponse {
    let exercise = TrainingPlan::new(
        req.day.clone(),
        req.theme.clone(),
        req.estimated_time.clone(),
        req.schedule_days.clone(),
    );
    let insert;
    {
        match training_plan_db::post_training(data.client.clone(), exercise).await {
            Ok(training_plan) => insert = training_plan,
            Err(err) => {
                eprintln!("CT Training: {:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        };
    }
    HttpResponse::Ok().json(insert)
}

#[tracing::instrument(name = "Controller: Get Training By Id")]
pub async fn get_training_by_id(data: Data<AppState>, _id: Path<String>) -> HttpResponse {
    let exercise =
        match training_plan_db::get_training_by_id(data.client.clone(), _id.clone()).await {
            Ok(training) => training,
            Err(err) => {
                eprintln!(
                    "CT Training: Error failed to retrieve the exercise with _id: {:?}",
                    err
                );
                return HttpResponse::InternalServerError().finish();
            }
        };
    HttpResponse::Ok().json(&exercise)
}

#[tracing::instrument(name = "Controller: Update Training by Id")]
pub async fn update_training_by_id(
    data: Data<AppState>,
    _id: Path<String>,
    req: Json<TrainingPlan>,
) -> HttpResponse {
    let id = _id.into_inner();

    if id.is_empty() {
        eprintln!("CT Training: empty id");
        return HttpResponse::BadRequest().finish();
    };

    let training_data = TrainingPlan {
        _id: Some(ObjectId::parse_str(&id).unwrap()),
        day: req.day.clone(),
        theme: req.theme.clone(),
        estimated_time: req.estimated_time.clone(),
        schedule_days: req.schedule_days.clone(),
    };

    let updated_training =
        match training_plan_db::update_training(data.client.clone(), training_data).await {
            Ok(training) => training,
            Err(err) => {
                eprintln!("CT Training: Error failed to update the training {:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        };
    HttpResponse::Ok().json(&updated_training)
}

#[tracing::instrument(name = "Controller: Delete Training by Id")]
pub async fn delete_training_by_id(data: Data<AppState>, _id: Path<String>) -> HttpResponse {
    let deleted_training =
        match training_plan_db::delete_training_by_id(data.client.clone(), _id.clone()).await {
            Ok(training) => training,
            Err(err) => {
                eprintln!("CT Training: Error failed to delete the user {:?}", err);
                return HttpResponse::InternalServerError().finish();
            }
        };

    HttpResponse::Ok().json(&deleted_training)
}
