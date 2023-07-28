use std::net::SocketAddr;

use axum::Router;

pub async fn start(addr: &SocketAddr, router: Router) -> anyhow::Result<()> {
    log::info!("Start server at {addr}");
    axum::Server::bind(addr)
        .serve(router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    log::info!("Server stopped");
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to register Ctrl+C handler")
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to register termination signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    log::info!("Termination signal received");
}
