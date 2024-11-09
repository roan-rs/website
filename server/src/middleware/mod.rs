use crate::{app::AppState, middleware::session::handle_session};
use axum::{middleware::from_fn_with_state, Router};
use tower_http::add_extension::AddExtensionLayer;

pub mod session;

pub fn inject_middlewares(app: AppState, router: Router<()>) -> Router {
    let middlewares = tower::ServiceBuilder::new()
        .layer(from_fn_with_state(app.clone(), handle_session))
        .layer(AddExtensionLayer::new(app.clone()));

    router.layer(middlewares)
}
