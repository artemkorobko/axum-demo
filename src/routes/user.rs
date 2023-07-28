use axum::{routing, Router};

use crate::handlers;

pub fn route() -> Router {
    Router::new().route("/user", routing::post(handlers::user::create::handle))
}
