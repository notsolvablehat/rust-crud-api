use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    // A new app - main router.
    let app = Router::new()
        .route("/", get(|| async { "Yo" }))
        .route("/health", get(|| async { "I am healthy." }));

    let addr = String::from("0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Server running on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
