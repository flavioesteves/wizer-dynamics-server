use crate::database::database_configuration::{DBCollectionSettings, DBSettings};
use crate::models::training_plan::TrainingPlan;

use actix_web::Result;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    error::Error,
    results::{DeleteResult, UpdateResult},
    Client, Collection,
};

#[tracing::instrument(name = "Post Training", skip(client), fields(training))]
pub async fn post_training(client: Client, training: TrainingPlan) -> Result<(), Error> {
    let trainings_collection: Collection<Document> = client
        .database(&DBSettings::default().name)
        .collection(DBCollectionSettings::TrainingPlans.value());
    let mut training_doc =
        bson::to_document(&training).expect("DB: Failed to Convert Training to Document");
    training_doc.insert("_id", ObjectId::new());

    trainings_collection
        .insert_one(training_doc, None)
        .await
        .expect("Failed to inseert a new Training");

    Ok(())
}
#[tracing::instrument(name = "Get All Trainings")]
pub async fn get_all_trainings(client: Client) -> Result<Vec<TrainingPlan>, Error> {
    let trainings_collection: Collection<TrainingPlan> = client
        .database(&DBSettings::default().name)
        .collection(DBCollectionSettings::TrainingPlans.value());
    let mut trainings: Vec<TrainingPlan> = Vec::new();

    let mut cursors = trainings_collection
        .find(doc! {}, None)
        .await
        .expect("Error getting list of Exercises");

    while let Some(training) = cursors
        .try_next()
        .await
        .expect("Error mapping cursor exercise")
    {
        trainings.push(training)
    }
    Ok(trainings)
}

#[tracing::instrument(name = "Get Training By Id", skip(client), fields(_id))]
pub async fn get_training_by_id(
    client: Client,
    _id: String,
) -> Result<Option<TrainingPlan>, Error> {
    let trainings_collection: Collection<TrainingPlan> = client
        .database(&DBSettings::default().name)
        .collection(DBCollectionSettings::TrainingPlans.value());

    if let Ok(obj_id) = ObjectId::parse_str(&_id) {
        if let Some(training) = trainings_collection
            .find_one(doc! {"_id": obj_id}, None)
            .await?
        {
            return Ok(Some(training));
        }
    }
    Ok(None)
}

#[tracing::instrument(name = "Update Training", skip(client) fields(training))]
pub async fn update_training(
    client: Client,
    training: TrainingPlan,
) -> Result<UpdateResult, Error> {
    let trainings_collection: Collection<TrainingPlan> = client
        .database(&DBSettings::default().name)
        .collection(DBCollectionSettings::TrainingPlans.value());
    let filter = doc! {"_id": training._id};
    let new_doc = doc! {
        "$set": {
            "day": training.day,
            "theme": training.theme,
            "estimated_time": training.estimated_time,
            "schedule_days": training.schedule_days,
        },
    };

    let training_updated = trainings_collection
        .update_one(filter, new_doc, None)
        .await
        .expect("Training DB Error to update the training");

    Ok(training_updated)
}

#[tracing::instrument(name = "Delete Training By Id", skip(client), fields(_id))]
pub async fn delete_training_by_id(
    client: Client,
    _id: String,
) -> Result<Option<TrainingPlan>, Error> {
    let trainings_collection: Collection<TrainingPlan> = client
        .database(&DBSettings::default().name)
        .collection(DBCollectionSettings::TrainingPlans.value());
    if let Ok(obj_id) = ObjectId::parse_str(&_id) {
        if let DeleteResult {
            deleted_count: 1, ..
        } = trainings_collection
            .delete_one(doc! {"_id": obj_id}, None)
            .await?
        {
            return Ok(Some(TrainingPlan {
                day: "".to_string(),
                theme: "".to_string(),
                estimated_time: "".to_string(),
                schedule_days: "".to_string(),
                _id: None,
            }));
        }
    }
    Ok(None)
}
