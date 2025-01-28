use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
#[derive(Debug)]
pub enum LoginIdentifier {
    Username(String),
    Email(String),
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub identifier: LoginIdentifier,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub id: u64,
    pub token: String,
    pub message: String,
}
