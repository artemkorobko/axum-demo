use std::sync::{Arc, RwLock};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::database;

#[derive(serde::Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[axum::debug_handler]
pub async fn handle(
    users: State<Arc<RwLock<database::Users>>>,
    creds: Json<Credentials>,
) -> impl IntoResponse {
    if let Ok(mut db) = users.write() {
        try_create_user(&mut db, &creds)
    } else {
        log::error!("Faild to get write access to users database");
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

fn try_create_user(db: &mut database::Users, creds: &Credentials) -> StatusCode {
    if can_register_email(db, &creds.email) {
        create_user(db, &creds)
    } else {
        StatusCode::CONFLICT
    }
}

fn can_register_email(db: &database::Users, email: &str) -> bool {
    db.find_by_email(email).is_none()
}

fn create_user(db: &mut database::Users, creds: &Credentials) -> StatusCode {
    if let Err(err) = db.create(&creds.email, &creds.password) {
        log::error!("{}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    } else {
        StatusCode::CREATED
    }
}
