use std::sync::Arc;

use axum::{routing, Router};

use crate::{database, handlers};

pub fn route(users: Arc<database::Users>) -> Router {
    Router::new()
        .route("/login", routing::post(handlers::user::login::handle))
        .with_state(users)
}
