use crate::{
    handler,
    middleware::{cors, trace},
    AppState,
};
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn init(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(handler::demo::root))
        .route("/users", post(handler::demo::create_user))
        .layer(cors::cors())
        .layer(trace::trace())
        .with_state(state)
}
