use axum::{routing, Router};

use crate::handlers;

pub fn route() -> Router {
    Router::new().route("/health", routing::get(handlers::health::handle))
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
    async fn return_service_info() {
        let request = http::Request::builder()
            .uri("/health")
            .method(http::Method::GET)
            .body(Body::empty())
            .unwrap();

        let mut response = route().oneshot(request).await.unwrap();

        assert_eq!(response.status(), http::StatusCode::OK);
        let response_body = response.data().await.unwrap().unwrap();
        let expected_body = r#"{"service":"axum-test","version":"0.1.0"}"#;
        assert_eq!(response_body, expected_body);
    }
}
