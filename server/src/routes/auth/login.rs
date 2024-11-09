use crate::{app::AppState, json_response, middleware::session::SessionExtension};
use axum::Json;
use oauth2::Scope;
use serde_json::{json, Value};
use tracing::debug;

pub async fn login(app: AppState, session: SessionExtension) -> Json<Value> {
    let (url, state) = app
        .github_auth
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scope(Scope::new("read:org".to_string()))
        .url();

    session.insert("github_oauth_state".to_string(), state.secret().to_string());
    debug!("Generated OAuth state: {}", state.secret());

    json_response!({ "url": url.to_string(), "state": state.secret().to_string() })
}
