use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::user::{SignupRequest, UserResponse},
    state::AppState,
};

pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignupRequest>,
) -> Result<impl IntoResponse, AppError> {
    let pass_hash = match bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST) {
        Ok(hash) => hash,
        Err(e) => {
            tracing::error!(error = ?e, "failed to hash password");
            return Err(AppError::InternalError);
        }
    };

    let id = Uuid::new_v4();

    match sqlx::query!(
        "insert into users (id, email, pass_hash) values ($1, $2, $3)",
        id,
        payload.email,
        pass_hash
    )
    .execute(&state.db)
    .await
    {
        Ok(_) => {
            let res = UserResponse {
                id,
                email: payload.email,
            };

            Ok((StatusCode::CREATED, Json(res)))
        }

        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            tracing::warn!(email = %payload.email, "signup attempt with existing email");
            Err(AppError::EmailTaken)
        }

        Err(e) => {
            tracing::error!(error = ?e, "failed to insert new user");
            Err(AppError::InternalError)
        }
    }
}
