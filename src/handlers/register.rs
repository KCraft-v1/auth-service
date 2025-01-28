use warp::reply::Json;

use crate::models::{RegisterRequest, RegisterResponse};
/*
    TODO:
    - Implement proper validation
    - Implement proper error handling
    - Implement proper password hashing
    - Save user to database if validation passes
*/
pub fn handle_register_user(register_request: RegisterRequest) -> Json {
    warp::reply::json(&RegisterResponse {
        id: 1, // This will get fetched from the DB in the future
        username: register_request.username,
        email: register_request.email,
        message: "User created successfully!".to_string(),
    })
}
