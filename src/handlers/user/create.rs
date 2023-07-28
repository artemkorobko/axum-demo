use axum::{http::StatusCode, response::IntoResponse, Json};

#[derive(serde::Deserialize)]
pub struct Credentials {
    email: String,
    password: String,
}

#[axum::debug_handler]
pub async fn handle(_creds: Json<Credentials>) -> impl IntoResponse {
    StatusCode::CREATED
}
