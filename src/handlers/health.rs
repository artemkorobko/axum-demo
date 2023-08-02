use axum::{response::IntoResponse, Json};

#[derive(serde::Serialize)]
struct Response<'a> {
    service: &'a str,
    version: &'a str,
}

impl<'a> Response<'a> {
    pub fn from_env() -> Self {
        Self {
            service: std::env!("CARGO_PKG_NAME"),
            version: std::env!("CARGO_PKG_VERSION"),
        }
    }
}

#[axum::debug_handler]
pub async fn handle() -> impl IntoResponse {
    Json(Response::from_env())
}

#[cfg(test)]
mod tests {
    use axum::{
        body::{Body, HttpBody},
        http,
    };
    use tower::ServiceExt;

    use crate::{database, routes};

    #[tokio::test]
    async fn return_service_info() {
        let request = http::Request::builder()
            .uri("/health")
            .method(http::Method::GET)
            .body(Body::empty())
            .unwrap();

        let mut response = routes::build(database::Users::default())
            .oneshot(request)
            .await
            .unwrap();

        assert_eq!(response.status(), http::StatusCode::OK);
        let response_body = response.data().await.unwrap().unwrap();
        let expected_body = r#"{"service":"axum-demo","version":"0.1.0"}"#;
        assert_eq!(response_body, expected_body);
    }
}
