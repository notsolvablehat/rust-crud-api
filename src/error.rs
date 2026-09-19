use axum::{http::StatusCode, response::IntoResponse};

pub enum AppError {
    EmailTaken,
    InternalError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::EmailTaken => (StatusCode::CONFLICT, "Email already registered"),
            AppError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong. Please try again.",
            ),
        };

        (status, message).into_response()
    }
}
