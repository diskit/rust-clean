use axum::{Router, routing::get, Server};

#[tokio::main]
async fn main() {
    
    let router = Router::new()
        .route("/", get(|| async { "ok" }));

    Server::bind(&"0.0.0.0:10090".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap()
}
