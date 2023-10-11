use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct User {
    pub email: String,
    pub password: String,
    pub _id: Option<ObjectId>,
    pub updated: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        email: String,
        password: String,
        updated: DateTime<Utc>,
        created_at: DateTime<Utc>,
    ) -> User {
        User {
            email,
            password,
            updated,
            created_at,
            _id: None, //Omit the id field
        }
    }
}
