use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use std::{
    borrow::Cow,
    fmt,
    fmt::{Display, Formatter},
};
use tracing::error;

pub fn custom(status: StatusCode, detail: impl Into<Cow<'static, str>>) -> ApiError {
    ApiError {
        status,
        detail: detail.into(),
    }
}

#[derive(Debug, Clone)]
pub struct ApiError {
    status: StatusCode,
    detail: Cow<'static, str>,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.status, self.detail)
    }
}

pub fn not_found() -> ApiError {
    custom(StatusCode::NOT_FOUND, "Not Found")
}

impl ApiError {
    pub fn response(&self) -> Response {
        let json = json!({ "errors": [{ "detail": self.detail }], "status": self.status.as_u16(), "success": false });

        error!("{}: {}", self.status.as_u16(), self.detail);

        (self.status, Json(json)).into_response()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        self.response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
