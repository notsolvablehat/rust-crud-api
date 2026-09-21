use axum::{Json, extract::State, response::IntoResponse};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::{
    auth::Claims,
    error::AppError,
    models::user::{LoginRequest, LoginResponse, User},
    state::AppState,
};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let maybe_user =
        match sqlx::query_as!(User, "select * from users where email = $1", payload.email)
            .fetch_optional(&state.db)
            .await
        {
            Ok(u) => u,
            Err(e) => {
                tracing::error!(error = ?e, "failed to query user by email");
                return Err(AppError::InternalError);
            }
        };

    let user = match maybe_user {
        Some(u) => u,
        None => {
            tracing::warn!(email = %payload.email, "login attempt for unknown email");
            return Err(AppError::InvalidCredentials);
        }
    };

    let password_ok = match bcrypt::verify(&payload.password, &user.pass_hash) {
        Ok(matches) => matches,
        Err(e) => {
            tracing::error!(error = ?e, "failed to verify password hash");
            return Err(AppError::InternalError);
        }
    };

    if !password_ok {
        tracing::warn!(user_id = %user.id, "login attempt with wrong password");
        return Err(AppError::InvalidCredentials);
    }

    let claims = Claims {
        sub: user.id,
        exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
    };

    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    ) {
        Ok(t) => t,
        Err(e) => {
            tracing::error!(error = ?e, "failed to encode jwt");
            return Err(AppError::InternalError);
        }
    };

    Ok(Json(LoginResponse { token }))
}
