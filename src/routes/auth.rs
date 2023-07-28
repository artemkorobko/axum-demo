use axum::{routing, Router};

use crate::handlers;

pub fn route() -> Router {
    Router::new().route("/login", routing::post(handlers::user::login::handle))
}
