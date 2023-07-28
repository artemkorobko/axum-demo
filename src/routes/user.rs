use std::sync::Arc;

use axum::{routing, Router};

use crate::{database, handlers};

pub fn route(users: Arc<database::Users>) -> Router {
    Router::new()
        .route("/user", routing::post(handlers::user::create::handle))
        .with_state(users)
}
