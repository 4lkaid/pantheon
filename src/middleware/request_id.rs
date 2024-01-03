use axum::http::HeaderName;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};

pub fn set_request_id() -> SetRequestIdLayer<MakeRequestUuid> {
    SetRequestIdLayer::new(HeaderName::from_static("x-request-id"), MakeRequestUuid)
}

pub fn propagate_request_id() -> PropagateRequestIdLayer {
    PropagateRequestIdLayer::new(HeaderName::from_static("x-request-id"))
}
