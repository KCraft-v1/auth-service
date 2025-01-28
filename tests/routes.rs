use auth_service::models::{RegisterRequest, RegisterResponse};
use auth_service::server;
use warp::test::request;

async fn setup_server(
) -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    server::register_routes()
}

#[tokio::test]
async fn test_register_route() {
    let server = setup_server().await;

    let new_user = RegisterRequest {
        username: "user".to_string(),
        email: "".to_string(),
        password: "".to_string(),
    };

    let res = request()
        .method("POST")
        .path("/register")
        .json(&new_user)
        .reply(&server)
        .await;

    assert_eq!(res.status(), 200);

    let user: RegisterResponse = serde_json::from_slice(res.body()).unwrap();

    assert_eq!(user.id, 1);
}
