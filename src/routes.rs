use axum::{routing, Router};

mod health;

pub fn build() -> Router {
    Router::new().route("/health", routing::get(health::handle))
}
