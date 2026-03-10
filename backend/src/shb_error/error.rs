use axum::{http::StatusCode, response::IntoResponse, response::Response, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct BackendErrorData {
    code: String,
    message: String,
}

#[derive(Clone, Debug)]
pub enum BackendError {
    Conflict(String),
    _CustomMessage(String),
    NotAuthorized(String),
    BadRequest(String),
    NotFound(String),
    InternalServerError(String),
    UnprocessableEntity(String),
}

impl IntoResponse for BackendError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg),
            Self::Conflict(msg) => (StatusCode::CONFLICT, "conflict", msg),
            Self::InternalServerError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_server_error",
                msg,
            ),
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg),
            Self::UnprocessableEntity(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "unprocessable_entity",
                msg,
            ),
            Self::NotAuthorized(msg) => (StatusCode::UNAUTHORIZED, "not_authorised", msg),
            Self::_CustomMessage(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg),
        };

        (
            status,
            Json(BackendErrorData {
                code: code.to_string(),
                message: message,
            }),
        )
            .into_response()
    }
}
