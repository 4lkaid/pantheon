use super::{DIRECT_CONNECT_IP, X_FORWARDED_FOR, X_REAL_IP, X_REQUEST_ID};
use axum::{extract::Request, http::HeaderName};
use std::net::SocketAddr;
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{MakeSpan, TraceLayer},
};
use tracing::Span;

#[derive(Clone)]
pub struct CustomMakeSpan;

impl<B> MakeSpan<B> for CustomMakeSpan {
    fn make_span(&mut self, req: &Request<B>) -> Span {
        let method = req.method();
        let uri = req.uri();
        let x_request_id = req
            .headers()
            .get(HeaderName::from_static(X_REQUEST_ID))
            .and_then(|value| value.to_str().ok())
            .unwrap_or("N/A");
        let direct_connect_ip = req
            .extensions()
            .get::<axum::extract::connect_info::ConnectInfo<SocketAddr>>()
            .map(|addr| addr.ip().to_string())
            .unwrap_or("N/A".to_string());
        let x_forwarded_for = req
            .headers()
            .get(HeaderName::from_static(X_FORWARDED_FOR))
            .and_then(|value| value.to_str().ok())
            .unwrap_or("N/A");
        let x_real_ip = req
            .headers()
            .get(HeaderName::from_static(X_REAL_IP))
            .and_then(|value| value.to_str().ok())
            .unwrap_or("N/A");
        tracing::debug_span!(
            "request",
            %method,
            %uri,
            {X_REQUEST_ID} = %x_request_id,
            {DIRECT_CONNECT_IP} = %direct_connect_ip,
            {X_FORWARDED_FOR} = %x_forwarded_for,
            {X_REAL_IP} = %x_real_ip
        )
    }
}

pub fn trace() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>, CustomMakeSpan> {
    TraceLayer::new_for_http().make_span_with(CustomMakeSpan)
}
