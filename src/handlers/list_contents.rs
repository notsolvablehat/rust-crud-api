use axum::{Json, extract::State};

use crate::{
    auth::AuthUser, error::AppError, models::file::FileRecord, models::file::FileResponse,
    state::AppState,
};

pub async fn list_contents(
    auth_user: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<FileResponse>>, AppError> {
    let files = match sqlx::query_as!(
        FileRecord,
        "select * from files where user_id = $1 order by uploaded_at desc",
        auth_user.user_id
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(f) => f,
        Err(e) => {
            tracing::error!(error = ?e, "failed to list files for user");
            return Err(AppError::InternalError);
        }
    };

    let response = files
        .into_iter()
        .map(|f| FileResponse {
            id: f.id,
            original_file_name: f.original_file_name,
            content_type: f.content_type,
            file_size: f.file_size,
            uploaded_at: f.uploaded_at,
        })
        .collect();

    Ok(Json(response))
}
