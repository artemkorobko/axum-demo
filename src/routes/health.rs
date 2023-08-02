use axum::{routing, Router};

use crate::handlers;

pub fn route() -> Router {
    Router::new().route("/health", routing::get(handlers::health::handle))
}
