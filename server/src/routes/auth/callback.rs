use axum::extract::{FromRequestParts, Query};
use axum::Json;
use http::request::Parts;
use oauth2::reqwest::http_client;
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::runtime::Handle;
use tracing_subscriber::fmt::format::json;
use crate::app::AppState;
use crate::errors::ApiResult;
use crate::middleware::session::SessionExtension;
use crate::results::user::User;

#[derive(Clone, Debug, Deserialize, FromRequestParts)]
#[from_request(via(Query))]
pub struct AuthorizeQuery {
    code: AuthorizationCode,
    state: CsrfToken,
}

pub async fn authorize(
    query: AuthorizeQuery,
    app: AppState,
    session: SessionExtension,
    req: Parts,
) -> ApiResult<Json<User>> {}