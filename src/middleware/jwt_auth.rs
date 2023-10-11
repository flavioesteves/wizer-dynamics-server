use core::fmt;
use std::future::{ready, Ready};

use actix_web::{
    dev::Payload, error::ErrorUnauthorized, http, web, Error as ActixWebError, FromRequest,
    HttpMessage, HttpRequest,
};

use jsonwebtoken::{decode, DecodingKey, Validation};
use mongodb::bson::oid::ObjectId;
use serde::Serialize;

use crate::middleware::jwt_model::{AppState, TokenClaims};

#[derive(Debug, Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self).expect("Expected a error message to display")
        )
    }
}

pub struct JwtMiddleware {
    pub user_id: Option<ObjectId>,
}

impl FromRequest for JwtMiddleware {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let data = req
            .app_data::<web::Data<AppState>>()
            .expect("Error not found a Appstate");

        let token = req
            .cookie("token")
            .map(|cookie| cookie.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|header| {
                        header
                            .to_str()
                            .expect("Error header not found")
                            .split_at(7)
                            .1
                            .to_string()
                    })
            });

        if token.is_none() {
            let json_error = ErrorResponse {
                status: "fail".to_string(),
                message: "You are not logged in, please provide a tokem".to_string(),
            };
            return ready(Err(ErrorUnauthorized(json_error)));
        }

        let claims = match decode::<TokenClaims>(
            &token.expect("Error token not found"),
            &DecodingKey::from_secret(data.env.jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(cookie) => cookie.claims,
            Err(_) => {
                let json_error = ErrorResponse {
                    status: "fail".to_string(),
                    message: "Invalid Token".to_string(),
                };
                return ready(Err(ErrorUnauthorized(json_error)));
            }
        };

        let user_id =
            ObjectId::parse_str(claims.sub.as_str()).expect("Error sub not found on token claims");
        req.extensions_mut().insert::<ObjectId>(user_id.to_owned());

        ready(Ok(JwtMiddleware {
            user_id: Some(user_id),
        }))
    }
}
