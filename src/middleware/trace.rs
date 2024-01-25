use super::{DIRECT_CONNECT_IP, X_FORWARDED_FOR, X_REAL_IP, X_REQUEST_ID};
use axum::{extract::Request, http::HeaderName};
use std::net::SocketAddr;
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{MakeSpan, TraceLayer},
};
use tracing::{Level, Span};

const DEFAULT_MESSAGE_LEVEL: Level = Level::DEBUG;

#[derive(Debug, Clone)]
pub struct CustomMakeSpan {
    level: Level,
    include_headers: bool,
}

impl CustomMakeSpan {
    pub fn new() -> Self {
        Self {
            level: DEFAULT_MESSAGE_LEVEL,
            include_headers: false,
        }
    }

    pub fn level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    pub fn include_headers(mut self, include_headers: bool) -> Self {
        self.include_headers = include_headers;
        self
    }
}

impl<B> MakeSpan<B> for CustomMakeSpan {
    fn make_span(&mut self, req: &Request<B>) -> Span {
        let direct_connect_ip = req
            .extensions()
            .get::<axum::extract::connect_info::ConnectInfo<SocketAddr>>()
            .map(|addr| addr.ip().to_string())
            .unwrap_or("N/A".to_string());
        macro_rules! make_span {
            ($level:expr) => {
                if self.include_headers {
                    tracing::span!(
                        $level,
                        "request",
                        {DIRECT_CONNECT_IP} = %direct_connect_ip,
                        method = %req.method(),
                        uri = %req.uri(),
                        version = ?req.version(),
                        headers = ?req.headers(),
                    )
                } else {
                    tracing::span!(
                        $level,
                        "request",
                        {DIRECT_CONNECT_IP} = %direct_connect_ip,
                        {X_FORWARDED_FOR} = %req.headers().get(HeaderName::from_static(X_FORWARDED_FOR)).and_then(|value| value.to_str().ok()).unwrap_or("N/A"),
                        {X_REAL_IP} = %req.headers().get(HeaderName::from_static(X_REAL_IP)).and_then(|value| value.to_str().ok()).unwrap_or("N/A"),
                        {X_REQUEST_ID} = %req.headers().get(HeaderName::from_static(X_REQUEST_ID)).and_then(|value| value.to_str().ok()).unwrap_or("N/A"),
                        method = %req.method(),
                        uri = %req.uri(),
                        version = ?req.version(),
                    )
                }
            }
        }

        match self.level {
            Level::ERROR => make_span!(Level::ERROR),
            Level::WARN => make_span!(Level::WARN),
            Level::INFO => make_span!(Level::INFO),
            Level::DEBUG => make_span!(Level::DEBUG),
            Level::TRACE => make_span!(Level::TRACE),
        }
    }
}

pub fn trace() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>, CustomMakeSpan> {
    TraceLayer::new_for_http().make_span_with(CustomMakeSpan::new())
}
