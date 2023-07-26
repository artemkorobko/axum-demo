use std::net::SocketAddr;

use anyhow::Context;
use axum::Router;

pub async fn start(addr: &SocketAddr, router: Router) -> anyhow::Result<()> {
    axum::Server::bind(addr)
        .serve(router.into_make_service())
        .await
        .context("Serving error")
}
