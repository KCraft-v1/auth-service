use auth_service::server;

#[tokio::main]
async fn main() {
    println!("Server started at: http://localhost:8080");
    warp::serve(server::register_routes())
        .run(([127, 0, 0, 1], 8080))
        .await;
}
