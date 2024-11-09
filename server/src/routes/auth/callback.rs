use axum::extract::{FromRequestParts, Query};
use oauth2::{AuthorizationCode, CsrfToken};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, FromRequestParts)]
#[from_request(via(Query))]
pub struct AuthorizeQuery {
    code: AuthorizationCode,
    state: CsrfToken,
}

// pub async fn authorize(
//     query: AuthorizeQuery,
//     app: AppState,
//     session: SessionExtension,
//     req: Parts,
// ) -> ApiResult<Json<User>> {}
