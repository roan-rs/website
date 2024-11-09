use std::sync::Arc;
use axum::http::{Method, StatusCode};
use axum::response::IntoResponse;
use axum::Router;
use crate::app::{App, AppState};
use crate::errors::not_found;

pub fn build_router(app: AppState) -> Router<()> {
    let mut router = Router::new();

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

    build_router(app_state)
}