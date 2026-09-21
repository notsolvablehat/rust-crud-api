use axum::{http::StatusCode, response::IntoResponse};

pub enum AppError {
    EmailTaken,
    InternalError,
    InvalidCredentials,
    Unauthorized,
    NotFound,
    BadRequest,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::EmailTaken => (StatusCode::CONFLICT, "Email already registered"),
            AppError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong. Please try again.",
            ),
            AppError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid Email or Password"),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Invalid Credentials."),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Couldn't find the requested item."),
            AppError::BadRequest => (
                StatusCode::BAD_REQUEST,
                "Bad request. Try changing fields according to API documentation.",
            ),
        };

        (status, message).into_response()
    }
}
