use axum::{
    routing::post,
    Router,
};
use crate::handlers::tasks::webhook;

pub fn routes() -> Router {
    Router::new()
        .route("/webhook", post(webhook))
}
