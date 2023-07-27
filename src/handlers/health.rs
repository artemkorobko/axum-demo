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

pub async fn handle() -> impl IntoResponse {
    Json(Response::from_env())
}
