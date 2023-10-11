use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TrainingPlan {
    pub day: String,
    pub theme: String,
    pub estimated_time: String,
    pub schedule_days: String,
    #[serde(skip_serializing_if = "Option::is_none")] // Skip serializing if _id is None
    #[serde(default)] // Set default value to None
    pub _id: Option<ObjectId>,
}

impl TrainingPlan {
    pub fn new(
        day: String,
        theme: String,
        estimated_time: String,
        schedule_days: String,
    ) -> TrainingPlan {
        TrainingPlan {
            day,
            theme,
            estimated_time,
            schedule_days,
            _id: None,
        }
    }
}
