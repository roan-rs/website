use crate::{
    app::{App, AppState},
    errors::not_found,
    middleware::inject_middlewares,
    routes::auth::login::login,
};
use axum::{
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::sync::Arc;
// use crate::routes::auth::callback::authorize;

pub fn build_router(app: AppState) -> Router<()> {
    let router = Router::new().route("/api/auth/login", get(login));
    // .route("/api/auth/callback", get(authorize));

    router
        .fallback(|method: Method| async move {
            match method {
                Method::HEAD => StatusCode::NOT_FOUND.into_response(),
                _ => not_found().into_response(),
            }
        })
        .with_state(app)
}

pub fn build_handler(app: Arc<App>) -> Router<()> {
    let app_state = AppState(app);

    let router = build_router(app_state.clone());
    inject_middlewares(app_state, router)
}
