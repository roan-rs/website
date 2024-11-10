mod app;
mod errors;
mod middleware;
mod response;
mod results;
mod router;
mod routes;
mod shutdown;
mod tracing;
pub mod var;
mod db;

use crate::{app::App, router::build_handler, shutdown::shutdown_signal};
use ::tracing::info;
use anyhow::Result;
use axum::ServiceExt;
use std::{env::args, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tracing_subscriber::filter::LevelFilter;

fn main() -> Result<()> {
    let is_debug = args().any(|arg| arg == "--debug");
    tracing::init_with_default_level(if is_debug {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    });

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
            .with_graceful_shutdown(shutdown_signal())
            .await?;

        Ok::<(), anyhow::Error>(())
    })?;

    info!("Server has gracefully shutdown!");

    Ok(())
}
