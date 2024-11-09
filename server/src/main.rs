mod tracing;
pub mod var;
mod app;
mod router;
mod errors;
mod shutdown;
mod routes;
mod middleware;
mod results;
mod respose;

use std::net::SocketAddr;
use std::sync::Arc;
use ::tracing::{info, warn};
use anyhow::Result;
use axum::ServiceExt;
use tokio::net::TcpListener;
use crate::app::{App, AppState};
use crate::router::{build_handler, build_router};
use crate::shutdown::shutdown_signal;

fn main() -> Result<()> {
    tracing::init();

    let app = Arc::new(App::from_env()?);

    let mut builder = tokio::runtime::Builder::new_multi_thread();
    builder.enable_all();
    builder.worker_threads(4);

    let rt = builder.build()?;
    let router = build_handler(app.clone());

    let service = router.into_make_service_with_connect_info::<SocketAddr>();

    rt.block_on(async {
        let listener = TcpListener::bind((app.ip, app.port)).await?;

        let addr = listener.local_addr()?;

        info!("Listening at http://{addr}");

        axum::serve(listener, service)
            .with_graceful_shutdown(shutdown_signal()).await?;

        Ok::<(), anyhow::Error>(())
    })?;

    info!("Server has gracefully shutdown!");

    Ok(())
}