use warp::reply::Json;

use crate::models::{LoginRequest, LoginResponse};

pub fn handle_user_login(login_request: LoginRequest) -> Json {
    println!(
        "User logged in with username: {:?}",
        login_request.identifier
    );
    warp::reply::json(&LoginResponse {
        id: 1,
        token: "PLACEHOLDER_JWT".to_string(),
        message: "User logged in successfully!".to_string(),
    })
}
