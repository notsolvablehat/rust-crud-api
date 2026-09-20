use axum::{extract::FromRequestParts, http::request::Parts};
use jsonwebtoken::{DecodingKey, Validation};
use uuid::Uuid;

use crate::{error::AppError, state::AppState};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

pub struct AuthUser {
    pub user_id: Uuid,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let header_val = match parts.headers.get("authorization") {
            Some(header) => match header.to_str() {
                Ok(s) => s,
                Err(_) => return Err(AppError::Unauthorized),
            },
            None => return Err(AppError::Unauthorized),
        };

        let token = match header_val.strip_prefix("Bearer ") {
            Some(t) => t,
            None => return Err(AppError::InvalidCredentials),
        };

        let claims = match jsonwebtoken::decode::<Claims>(
            token,
            &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
            &Validation::default(),
        ) {
            Ok(data) => data.claims,
            Err(_) => return Err(AppError::InvalidCredentials),
        };

        Ok(AuthUser {
            user_id: claims.sub,
        })
    }
}
