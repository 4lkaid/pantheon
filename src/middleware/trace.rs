// use tower_http::{
//     classify::{ServerErrorsAsFailures, SharedClassifier},
//     trace::{DefaultMakeSpan, TraceLayer},
// };

// pub fn trace() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
//     TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::new().include_headers(true))
// }

use super::X_REQUEST_ID;
use axum::{extract::Request, http::HeaderName};
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
        tracing::debug_span!("request", %method, %uri, {X_REQUEST_ID} = %x_request_id)
    }
}

pub fn trace() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>, CustomMakeSpan> {
    TraceLayer::new_for_http().make_span_with(CustomMakeSpan)
}
