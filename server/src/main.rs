mod tracing;
pub mod var;
mod app;

use std::net::SocketAddr;
use ::tracing::{info, warn};
use anyhow::Result;
use tokio::net::TcpListener;
use crate::app::App;

fn main() -> Result<()> {
    tracing::init();

    let app = App::from_env()?;

    let mut builder = tokio::runtime::Builder::new_multi_thread();
    builder.enable_all();
    builder.worker_threads(4);

    let rt = builder.build()?;

    rt.block_on(async {
        let listener = TcpListener::bind((app.ip, app.port)).await?;

        let addr = listener.local_addr()?;

        info!("Listening at http://{addr}");

        Ok::<(), anyhow::Error>(())
    })?;

    info!("Server has gracefully shutdown!");

    Ok(())
}