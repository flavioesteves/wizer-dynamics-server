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

#[tracing::instrument(name = "Post User")]
pub async fn post_user(request: Json<User>) -> Result<HttpResponse, Error> {
    let current_time = Utc::now();
    let user = User::new(
        request.email.clone(),
        request.password.clone(),
        current_time,
        current_time,
    );

    Ok(HttpResponse::Ok().json(json!(&user)))
}

#[tracing::instrument(name = "Controller: Get All Users")]
pub async fn get_all_users(data: Data<AppState>) -> HttpResponse {
    let users = match user_db::get_all_users(data.client.clone()).await {
        Ok(users) => users,
        Err(err) => {
            eprintln!("CT USER: Error failed to retrieve users {:?}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };
    HttpResponse::Ok().json(&users)
}

#[tracing::instrument(name = "Controller Get User By Id")]
pub async fn get_user_by_id(data: Data<AppState>, _id: Path<String>) -> HttpResponse {
    let user = match user_db::get_user_by_id(data.client.clone(), _id.clone()).await {
        Ok(user) => user,
        Err(err) => {
            eprintln!(
                "CT USER: Error failed to retrieve the user with _id {:?}",
                err
            );
            return HttpResponse::InternalServerError().finish();
        }
    };
    HttpResponse::Ok().json(&user)
}

#[tracing::instrument(name = "Controller Get User Logged")]
pub async fn get_user_logged(
    data: Data<AppState>,
    request: HttpRequest,
    _: jwt_auth::JwtMiddleware,
) -> HttpResponse {
    let user_id = *request
        .extensions()
        .get::<ObjectId>()
        .expect("Expected user id on the token");

    let user = match user_db::get_user_by_id(data.client.clone(), user_id.to_string()).await {
        Ok(user) => user,
        Err(err) => {
            eprintln!(
                "CT USER: Error failed to rectrieve the user with _id {:?}",
                err
            );
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Ok().json(&user)
}
