use axum::{
    extract::{Path, State},
    http::{StatusCode, header},
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{auth::AuthUser, error::AppError, models::file::FileRecord, state::AppState};

pub async fn download(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(file_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let file = match sqlx::query_as!(
        FileRecord,
        "select * from files where id = $1 and user_id = $2",
        file_id,
        auth_user.user_id
    )
    .fetch_optional(&state.db)
    .await
    {
        Ok(Some(f)) => f,
        Ok(None) => return Err(AppError::NotFound),
        Err(_) => return Err(AppError::InternalError),
    };

    let get_result = state
        .r2
        .get_object()
        .bucket(&state.r2_bucket)
        .key(&file.stored_file_name)
        .send()
        .await;

    let object = match get_result {
        Ok(o) => o,
        Err(_) => return Err(AppError::InternalError),
    };

    let bytes = match object.body.collect().await {
        Ok(b) => b.into_bytes(),
        Err(_) => return Err(AppError::InternalError),
    };

    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, file.content_type.clone()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", file.original_file_name),
            ),
        ],
        bytes,
    ))
}
