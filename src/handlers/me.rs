use axum::Json;
use serde::Serialize;

use crate::auth::AuthUser;

#[derive(Serialize)]
pub struct MeResponse {
    pub user_id: uuid::Uuid,
}

pub async fn me(auth_user: AuthUser) -> Json<MeResponse> {
    Json(MeResponse {
        user_id: auth_user.user_id,
    })
}
