use axum::extract::State;
use axum::routing::post;
use axum::{Router, routing::get};

mod auth;
mod db;
mod error;
mod handlers;
mod models;
mod state;
use state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = db::connect_db().await.expect("Failed to connect to DB.");
    let jwt = std::env::var("JWT_SECRET").expect("JWT_SECRET should be set.");

    let state = AppState {
        db: pool,
        jwt_secret: jwt,
    };

    let app = Router::new()
        .route("/", get(|| async { "Yo" }))
        .route("/health", get(|| async { "I am healthy." }))
        .route("/db-health", get(db_health))
        .route("/signup", post(handlers::signup::signup))
        .route("/login", post(handlers::login::login))
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
