use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Exercise {
    pub name: String,
    pub steps: String,
    pub video: String,
    pub photo: String,
    #[serde(skip_serializing_if = "Option::is_none")] // Skip serializing if _id is None
    #[serde(default)] // Set default value to None
    pub _id: Option<ObjectId>,
}

impl Exercise {
    pub fn new(name: String, steps: String, video: String, photo: String) -> Exercise {
        Exercise {
            name,
            steps,
            video,
            photo,
            _id: None, //Omit the id field
        }
    }
}
