use chrono::Utc;
use uuid::Uuid;

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub pass_hash: String,
    pub created_at: chrono::DateTime<Utc>,
}

#[derive(serde::Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
}

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub token: String,
}
