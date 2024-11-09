#[cfg(unix)]
pub async fn shutdown_signal() {
    use tokio::signal::unix::{signal, SignalKind};

    let interrupt = signal(SignalKind::interrupt()).expect("failed to install signal handler");
    let terminate = signal(SignalKind::terminate()).expect("failed to install signal handler");

    tokio::select! {
        _ = interrupt.recv() => {},
        _ = terminate.recv() => {},
    }
}

#[cfg(windows)]
pub async fn shutdown_signal() {
    use tokio::signal::windows::{ctrl_break, ctrl_c};

    let mut ctrl_c = ctrl_c().expect("failed to install CTRL+C handler");
    let mut ctrl_break = ctrl_break().expect("failed to install CTRL+BREAK handler");

    tokio::select! {
        _ = ctrl_c.recv() => {},
        _ = ctrl_break.recv() => {},
    }
}
