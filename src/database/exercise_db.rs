use crate::database::database_configuration::{DBCollectionSettings, DBSettings};
use actix_web::Result;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{self, doc, oid::ObjectId, Document},
    error::Error,
    results::{DeleteResult, UpdateResult},
    Client, Collection,
};

use crate::models::exercise::Exercise;

pub async fn post_exercise(client: Client, exercise: Exercise) -> Result<(), Error> {
    let exercises_collection: Collection<Document> = client
        .database(&DBSettings::default().name)
        .collection(DBCollectionSettings::Exercises.value());
    let mut exercise_doc =
        bson::to_document(&exercise).expect("DB: Failed to Convert Exercise to Document");
    exercise_doc.insert("_id", ObjectId::new());

    exercises_collection
        .insert_one(exercise_doc, None)
        .await
        .expect("Failed to inseert a new exercise");

    Ok(())
}

pub async fn get_all_exercises(client: Client) -> Result<Vec<Exercise>, Error> {
    let exercises_collection: Collection<Exercise> = client
        .database(&DBSettings::default().name)
        .collection(DBCollectionSettings::Exercises.value());
    let mut exercises: Vec<Exercise> = Vec::new();

    let mut cursors = exercises_collection
        .find(doc! {}, None)
        .await
        .expect("Error getting list of Exercises");

    while let Some(exercise) = cursors
        .try_next()
        .await
        .expect("Error mapping cursor exercise")
    {
        exercises.push(exercise)
    }
    Ok(exercises)
}

pub async fn get_exercise_by_id(client: Client, _id: String) -> Result<Option<Exercise>, Error> {
    let exercises_collection: Collection<Exercise> = client
        .database(&DBSettings::default().name)
        .collection(DBCollectionSettings::Exercises.value());

    if let Ok(obj_id) = ObjectId::parse_str(&_id) {
        if let Some(exercise) = exercises_collection
            .find_one(doc! {"_id": obj_id}, None)
            .await?
        {
            return Ok(Some(exercise));
        }
    }
    Ok(None)
}

pub async fn update_exercise(client: Client, exercise: Exercise) -> Result<UpdateResult, Error> {
    let exercises_collection: Collection<Exercise> = client
        .database(&DBSettings::default().name)
        .collection(DBCollectionSettings::Exercises.value());
    let filter = doc! {"_id": exercise._id};
    let new_doc = doc! {
        "$set": {
            "name": exercise.name,
            "steps": exercise.steps,
            "video": exercise.video,
            "photo" : exercise.photo,
        },
    };

    let exercise_updated = exercises_collection
        .update_one(filter, new_doc, None)
        .await
        .expect("Exercise DB Error to update the exercise");

    Ok(exercise_updated)
}

pub async fn delete_exercise_by_id(client: Client, _id: String) -> Result<Option<Exercise>, Error> {
    let exercises_collection: Collection<Exercise> = client
        .database(&DBSettings::default().name)
        .collection(DBCollectionSettings::Exercises.value());
    if let Ok(obj_id) = ObjectId::parse_str(&_id) {
        if let DeleteResult {
            deleted_count: 1, ..
        } = exercises_collection
            .delete_one(doc! {"_id": obj_id}, None)
            .await?
        {
            return Ok(Some(Exercise {
                name: "".to_string(),
                steps: "".to_string(),
                video: "".to_string(),
                photo: "".to_string(),
                _id: None,
            }));
        }
    }
    Ok(None)
}
