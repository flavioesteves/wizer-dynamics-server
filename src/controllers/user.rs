use actix_web::{
    web::{Data, Json, Path},
    Error, HttpMessage, HttpRequest, HttpResponse,
};

use chrono::Utc;
use mongodb::bson::oid::ObjectId;
use serde_json::json;

use crate::middleware::jwt_auth;
use crate::middleware::jwt_model::AppState;

use crate::database::user_db;
use crate::models::user::User;

pub async fn post_user(request: Json<User>) -> Result<HttpResponse, Error> {
    let current_time = Utc::now();
    let user = User::new(
        request.email.clone(),
        request.password.clone(),
        current_time,
        current_time,
    );

    Ok(HttpResponse::Ok().json(json!(user)))
}

pub async fn get_all_users(data: Data<AppState>) -> HttpResponse {
    let users = user_db::get_all_users(data.client.clone())
        .await
        .expect("CT USER: Error failed to retrieve users");

    HttpResponse::Ok().json(users)
}

pub async fn get_user_by_id(data: Data<AppState>, _id: Path<String>) -> HttpResponse {
    println!("User _id: {}", _id);
    let user = user_db::get_user_by_id(data.client.clone(), _id.clone())
        .await
        .expect("CT USER: Error failed to retrieve the user with _id");

    HttpResponse::Ok().json(user)
}

pub async fn get_user_logged(
    data: Data<AppState>,
    request: HttpRequest,
    _: jwt_auth::JwtMiddleware,
) -> HttpResponse {
    let ext = request.extensions();
    let user_id = ext
        .get::<ObjectId>()
        .expect("Expected user Id on the token");

    let user = user_db::get_user_by_id(data.client.clone(), user_id.to_string())
        .await
        .expect("CT USER: Error failed to rectrieve the user with _id");

    HttpResponse::Ok().json(user)
}
