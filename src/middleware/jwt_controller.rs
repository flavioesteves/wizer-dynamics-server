use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    web::{Data, Json},
    HttpResponse,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::bson::oid::ObjectId;
use serde_json::json;

use crate::database::user_db;
use crate::middleware::jwt_model::{AppState, LoginUserSchema, RegisterUserSchema, TokenClaims};
use crate::models::user::User;

pub async fn register(request: Json<RegisterUserSchema>, data: Data<AppState>) -> HttpResponse {
    let user_exists: bool =
        user_db::get_user_by_email(data.client.clone(), request.email.to_owned())
            .await
            .expect("Error on query the database")
            .is_some();

    if user_exists {
        return HttpResponse::Conflict()
            .json(json!({"status": "fail", "message": "User already exists with that email"}));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(request.password.as_bytes(), &salt)
        .expect("Error while parsing passord")
        .to_string();

    let current_time = Utc::now();

    let user_data: User = User {
        email: request.email.to_string(),
        password: hashed_password,
        _id: Some(ObjectId::new()),
        updated: current_time,
        created_at: current_time,
    };

    user_db::post_user(data.client.clone(), user_data.clone())
        .await
        .expect("Error on insert a new user");

    HttpResponse::Ok().json(json!({"status": "success", "data": user_data}))
}

pub async fn login(request: Json<LoginUserSchema>, data: Data<AppState>) -> HttpResponse {
    let user = user_db::get_user_by_email(data.client.clone(), request.email.to_owned())
        .await
        .expect("Error in retrieving the User");

    if user.is_none() {
        return HttpResponse::BadRequest()
            .json(json!({"status": "fail", "message": "Invalid email or password"}));
    }

    let loaded_user: User = user.clone().expect("Error in converting into User");
    let is_valid = loaded_user._id.as_ref().map_or(false, |_| {
        // Implement the Arg validation
        let parsed_hash =
            PasswordHash::new(&loaded_user.password).expect("Error on access the password");
        Argon2::default()
            .verify_password(request.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true)
    });

    if !is_valid {
        return HttpResponse::BadRequest()
            .json(json!({"status": "fail", "message": "Invalid email or password"}));
    }

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let user_id = loaded_user._id.expect("Erro Id not founded it");
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user_id.to_string(),
        iat,
        exp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .expect("Error to create a token");

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(60 * 60, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success", "token": token }))
}
