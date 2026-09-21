use aws_sdk_s3::primitives::ByteStream;
use axum::{
    Json,
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{auth::AuthUser, error::AppError, models::file::FileResponse, state::AppState};

pub async fn upload(
    auth_user: AuthUser,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let field = match multipart.next_field().await {
        Ok(Some(f)) => f,
        Ok(None) => return Err(AppError::BadRequest),
        Err(e) => {
            eprintln!("multipart next_field error: {:?}", e);
            return Err(AppError::InternalError);
        }
    };

    let original_file_name = field.file_name().unwrap_or("unnamed").to_string();
    let content_type = field
        .content_type()
        .unwrap_or("application/octect-stream")
        .to_string();

    let data = match field.bytes().await {
        Ok(b) => b,
        Err(e) => {
            eprintln!("field bytes error: {:?}", e);
            return Err(AppError::InternalError);
        }
    };

    let file_size = data.len() as i32;
    let file_id = Uuid::new_v4();
    let object_key = format!("{}/{}", auth_user.user_id, file_id);

    let put_result = state
        .r2
        .put_object()
        .bucket(&state.r2_bucket)
        .key(&object_key)
        .body(ByteStream::from(data.to_vec()))
        .content_type(&content_type)
        .send()
        .await;

    if let Err(e) = put_result {
        eprintln!("r2 put_object error: {:?}", e);
        return Err(AppError::InternalError);
    }

    let insert_result = sqlx::query!("insert into files (id, stored_file_name, original_file_name, content_type, file_size, user_id) values ($1, $2, $3, $4, $5, $6)", file_id, object_key, original_file_name, content_type, file_size, auth_user.user_id).execute(&state.db).await;

    match insert_result {
        Ok(_) => Ok((
            StatusCode::CREATED,
            Json(FileResponse {
                id: file_id,
                original_file_name,
                content_type,
                file_size,
                uploaded_at: chrono::Utc::now(),
            }),
        )),

        Err(e) => {
            eprintln!("db insert error: {:?}", e);
            Err(AppError::InternalError)
        }
    }
}
