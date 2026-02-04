use serde::{Deserialize, Serialize};

use crate::models::_entities::users;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub is_verified: bool,
    pub name: String,
    pub pid: String,
    pub token: String,
}

impl LoginResponse {
    #[must_use]
    pub fn new(user: &users::Model, token: &str) -> Self {
        Self {
            is_verified: user.email_verified_at.is_some(),
            name: user.name.clone(),
            pid: user.pid.to_string(),
            token: token.to_owned(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentResponse {
    pub email: String,
    pub name: String,
    pub pid: String,
}

impl CurrentResponse {
    #[must_use]
    pub fn new(user: &users::Model) -> Self {
        Self {
            email: user.email.clone(),
            name: user.name.clone(),
            pid: user.pid.to_string(),
        }
    }
}
