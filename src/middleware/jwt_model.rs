use crate::middleware::jwt_config::Config;
use mongodb::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub struct AppState {
    pub env: Config,
    pub client: Client,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub email: String,
    pub password: String,
}
