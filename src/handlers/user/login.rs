use axum::{response::IntoResponse, Json};

#[derive(serde::Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

#[axum::debug_handler]
pub async fn handle(_creds: Json<Credentials>) -> impl IntoResponse {
    "Authenticated"
}
