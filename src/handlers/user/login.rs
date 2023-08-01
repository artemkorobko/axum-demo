use std::sync::{Arc, RwLock};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{database, handlers::user::create::Credentials};

#[axum::debug_handler]
pub async fn handle(
    users: State<Arc<RwLock<database::Users>>>,
    creds: Json<Credentials>,
) -> impl IntoResponse {
    if let Ok(db) = users.write() {
        if let Some(user) = db.find_by_email(&creds.email) {
            if user.password == creds.password {
                StatusCode::OK
            } else {
                StatusCode::UNAUTHORIZED
            }
        } else {
            StatusCode::UNAUTHORIZED
        }
    } else {
        log::error!("Faild to get read access to users database");
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
