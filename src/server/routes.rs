use warp::reject::Rejection;
use warp::reply::Reply;
use warp::Filter;

use crate::handlers::handle_register_user;
use crate::models::RegisterRequest;

pub fn register_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("register")
        .and(warp::post())
        .and(warp::body::json())
        .map(|new_user: RegisterRequest| handle_register_user(new_user))
}
