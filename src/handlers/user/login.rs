use axum::{response::IntoResponse, Json};

use crate::handlers::user::create::Credentials;

#[axum::debug_handler]
pub async fn handle(_creds: Json<Credentials>) -> impl IntoResponse {
    "Authenticated"
}
