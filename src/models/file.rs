use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(sqlx::FromRow)]
pub struct FileRecord {
    pub id: Uuid,
    pub stored_file_name: String,
    pub original_file_name: String,
    pub content_type: String,
    pub file_size: i32,
    pub uploaded_at: DateTime<Utc>,
    pub user_id: Uuid,
}

#[derive(serde::Serialize)]
pub struct FileResponse {
    pub id: Uuid,
    pub original_file_name: String,
    pub content_type: String,
    pub file_size: i32,
    pub uploaded_at: DateTime<Utc>,
}
