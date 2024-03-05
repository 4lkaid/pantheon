use crate::{
    handler,
    middleware::{cors, request_id, request_response_logger, trace},
};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;

pub fn init() -> Router {
    Router::new()
        .route("/", get(handler::demo::root))
        .route("/users", post(handler::demo::create_user))
        .layer(middleware::from_fn(
            request_response_logger::print_request_response,
        ))
        // Its recommended to use tower::ServiceBuilder to apply multiple middleware at once, instead of calling layer (or route_layer) repeatedly.
        // ServiceBuilder works by composing all layers into one such that they run top to bottom.
        // Executing middleware top to bottom is generally easier to understand and follow mentally which is one of the reasons ServiceBuilder is recommended.
        .layer(
            ServiceBuilder::new()
                .layer(request_id::set_request_id())
                .layer(request_id::propagate_request_id())
                .layer(trace::trace())
                .layer(cors::cors()),
        )
}
