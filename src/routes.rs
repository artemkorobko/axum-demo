use axum::Router;

mod auth;
mod health;

pub fn build() -> Router {
    Router::new().merge(auth::route()).merge(health::route())
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
        let request = http::Request::builder()
            .uri("/")
            .method(http::Method::GET)
            .body(Body::empty())
            .unwrap();

        let mut response = build().oneshot(request).await.unwrap();

        assert_eq!(response.status(), http::StatusCode::NOT_FOUND);
        assert!(response.data().await.is_none());
    }
}
