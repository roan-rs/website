use crate::var::var;
use tracing::log::warn;
use tracing_subscriber::{filter::LevelFilter, prelude::*, EnvFilter, Layer};

pub fn init() {
    init_with_default_level(LevelFilter::INFO)
}

pub fn init_with_default_level(level: LevelFilter) {
    let env_filter = EnvFilter::builder()
        .with_default_directive(level.into())
        .from_env_lossy();

    let log_format = var("RUST_LOG_FORMAT")
        .inspect_err(|error| {
            warn!("Failed to read RUST_LOG_FORMAT, falling back to default: {error}")
        })
        .unwrap_or_default();

    let log_layer = match log_format.as_deref() {
        Some("json") => json_subscriber::fmt::layer()
            .flatten_event(true)
            .with_flat_span_list(true)
            .with_filter(env_filter)
            .boxed(),
        _ => tracing_subscriber::fmt::layer()
            .compact()
            .without_time()
            .with_filter(env_filter)
            .boxed(),
    };

    tracing_subscriber::registry().with(log_layer).init();
}
