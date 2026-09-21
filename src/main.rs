use axum::extract::State;
use axum::routing::post;
use axum::{Router, routing::get};

mod auth;
mod db;
mod error;
mod handlers;
mod models;
mod r2;
mod state;
use state::AppState;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let pool = db::connect_db().await.expect("Failed to connect to DB.");
    let jwt = std::env::var("JWT_SECRET").expect("JWT_SECRET should be set.");
    let r2 = r2::build_r2_client().await;
    let r2_bucket = std::env::var("R2_BUCKET_NAME").expect("R2_BUCKET_NAME should be set.");

    let state = AppState {
        db: pool,
        jwt_secret: jwt,
        r2_bucket,
        r2,
    };

    let app = Router::new()
        .route("/", get(|| async { "Yo" }))
        .route("/health", get(|| async { "I am healthy." }))
        .route("/db-health", get(db_health))
        .route("/signup", post(handlers::signup::signup))
        .route("/login", post(handlers::login::login))
        .route("/me", get(handlers::me::me))
        .route("/upload", post(handlers::upload::upload))
        .route("/download/{file_id}", get(handlers::download::download))
        .route(
            "/list-contents",
            get(handlers::list_contents::list_contents),
        )
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
