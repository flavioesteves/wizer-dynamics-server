use crate::database::database_configuration::{DBCollectionSettings, DBSettings};
use crate::models::user::User;
use actix_web::Result;
use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    error::Error,
    Client, Collection,
};
use std::option::Option;

/**
 * Retrive all users from the collection users
 * Sorted without any order
 */
pub async fn get_all_users(client: Client) -> Result<Vec<User>, Error> {
    let users_collection: Collection<User> = client
        .database(&DBSettings::default().name)
        .collection(DBCollectionSettings::Users.value());

    let mut users: Vec<User> = Vec::new();
    let mut cursors = users_collection
        .find(doc! {}, None)
        .await
        .expect("Error getting list of users");

    while let Some(user) = cursors
        .try_next()
        .await
        .expect("Error mapping through cursor")
    {
        users.push(user);
    }

    Ok(users)
}

/**
 * Retrieve a user by the parameter _id
 */
pub async fn get_user_by_id(client: Client, _id: String) -> Result<Option<User>, Error> {
    let users_collection: Collection<User> = client
        .database(&DBSettings::default().name)
        .collection(DBCollectionSettings::Users.value());

    if let Ok(obj_id) = ObjectId::parse_str(&_id) {
        if let Some(user) = users_collection
            .find_one(doc! {"_id": obj_id}, None)
            .await?
        {
            return Ok(Some(user));
        }
    }

    Ok(None)
}

pub async fn get_user_by_email(client: Client, email: String) -> Result<Option<User>, Error> {
    let users_collection: Collection<User> = client
        .database(&DBSettings::default().name)
        .collection(DBCollectionSettings::Users.value());

    if let Some(user) = users_collection
        .find_one(doc! {"email": email}, None)
        .await?
    {
        return Ok(Some(user));
    }

    Ok(None)
}

pub async fn post_user(client: Client, user: User) -> Result<(), Error> {
    let users_collection: Collection<User> = client
        .database(&DBSettings::default().name)
        .collection(DBCollectionSettings::Users.value());

    users_collection
        .insert_one(user, None)
        .await
        .expect("Failed to insert a new user");
    Ok(())
}
