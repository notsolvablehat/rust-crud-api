use axum::extract::State;
use axum::{Router, routing::get};

mod db;
mod state;

use state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = db::connect_db().await.expect("Failed to connect to DB.");

    let state = AppState { db: pool };

    let app = Router::new()
        .route("/", get(|| async { "Yo" }))
        .route("/health", get(|| async { "I am healthy." }))
        .route("/db-health", get(db_health))
        .with_state(state);

    let addr = String::from("0.0.0.0:3000");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Server running on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn db_health(State(state): State<AppState>) -> &'static str {
    match sqlx::query("select 1").execute(&state.db).await {
        Ok(_) => "db ok",
        Err(_) => "db unreachable",
    }
}
