use std::sync::{Arc, RwLock};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::database;

#[derive(serde::Deserialize)]
pub struct Credentials {
    email: String,
    password: String,
}

#[axum::debug_handler]
pub async fn handle(
    users: State<Arc<RwLock<database::Users>>>,
    creds: Json<Credentials>,
) -> impl IntoResponse {
    if let Ok(mut db) = users.write() {
        match db.create(&creds.email, &creds.password) {
            Ok(_) => StatusCode::CREATED,
            Err(err) => {
                log::error!("{}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    } else {
        log::error!("Faild to get write access to users database");
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
