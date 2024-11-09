use axum::Router;
use axum::middleware::from_fn_with_state;
use tower_http::add_extension::AddExtensionLayer;
use crate::app::AppState;
use crate::middleware::session::handle_session;

pub mod session;

pub fn inject_middlewares(
    app: AppState,
    router: Router<()>,
) -> Router {
    let middlewares = tower::ServiceBuilder::new()
        .layer(from_fn_with_state(app.clone(), handle_session))
        .layer(AddExtensionLayer::new(app.clone()));;

    router.layer(middlewares)
}