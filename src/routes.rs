use std::sync::{Arc, RwLock};

use axum::Router;

use crate::database;

mod auth;
mod health;
mod user;

pub fn build(users: database::Users) -> Router {
    let users_ptr = Arc::new(RwLock::new(users));
    Router::new()
        .merge(user::route(users_ptr.clone()))
        .merge(auth::route(users_ptr))
        .merge(health::route())
}

#[cfg(test)]
mod tests {
    use axum::{
        body::{Body, HttpBody},
        http,
    };
    use tower::ServiceExt;

    use super::*;

    #[tokio::test]
    async fn root_path_returns_404() {
        let users = database::Users::default();
        let request = http::Request::builder()
            .uri("/")
            .method(http::Method::GET)
            .body(Body::empty())
            .unwrap();

        let mut response = build(users).oneshot(request).await.unwrap();

        assert_eq!(response.status(), http::StatusCode::NOT_FOUND);
        assert!(response.data().await.is_none());
    }
}
