use warp::reject::Rejection;
use warp::reply::Reply;
use warp::Filter;

use crate::handlers::handle_register_user;
use crate::handlers::handle_user_login;
use crate::models::{LoginRequest, RegisterRequest};

pub fn register_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let register_route = warp::path("register")
        .and(warp::post())
        .and(warp::body::json())
        .map(|new_user: RegisterRequest| handle_register_user(new_user));

    let login_route = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .map(|user: LoginRequest| handle_user_login(user));

    register_route.or(login_route)
}
