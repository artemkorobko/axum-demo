use axum::Router;

mod health;

pub fn build() -> Router {
    Router::new().merge(health::route())
}
