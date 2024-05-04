use std::time::Duration;

use axum::http::{header, HeaderMap, Request, Response};
use opentelemetry::{propagation::Extractor, Context};
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{MakeSpan, OnResponse, TraceLayer},
};
use tracing::{field::Empty, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

pub(crate) fn layer(
) -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>, OtelMakeSpan, (), OtelOnResponse> {
    TraceLayer::new_for_http()
        .make_span_with(OtelMakeSpan)
        .on_request(())
        .on_response(OtelOnResponse)
}

/// Use this to create the root span for all requests coming into our web
/// server. You also want to make all attributes you intend to record values
/// for at a leter point in time.
#[derive(Clone, Copy)]
pub (crate) struct OtelMakeSpan;

impl<T> MakeSpan<T> for OtelMakeSpan {
    fn make_span(&mut self, request: &Request<T>) -> Span {
        // Gets parent/remote context or defaults to an unsampled context
        let context = get_context(request.headers());

        let user_agent = request
            .headers()
            .get(header::USER_AGENT)
            .map_or("", |v| v.to_str().unwrap_or(""));

        let host = request
            .headers()
            .get(header::HOST)
            .map_or("", |v| v.to_str().unwrap_or(""));

        let span = tracing::info_span!(
            "HTTP Request",
            http.host = host,
            http.method = %request.method(),
            http.route = %request.uri().path(),
            http.status_code = Empty,
            http.user_agent = user_agent
        );

        span.set_parent(context);

        span
    }
}

fn get_context(headers: &HeaderMap) -> Context {
    opentelemetry::global::get_text_map_propagator(|propagator| {
        propagator.extract(&OtelHeaderExtractor(headers))
    })
}

/// Update the status code and other interesting fields in the response
#[derive(Clone, Copy)]
pub (crate) struct OtelOnResponse;

impl<T> OnResponse<T> for OtelOnResponse {
    fn on_response(self, response: &Response<T>, _latency: Duration, span: &Span) {
        let status = response.status().as_u16().to_string();

        span.record("http.status_code", status);
    }
}

/// Create a struct to extract context from headers. This is necessary to enable
/// distributed tracing
struct OtelHeaderExtractor<'a>(pub &'a HeaderMap);

impl<'a> Extractor for OtelHeaderExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).and_then(|v| v.to_str().ok())
    }

    fn keys(&self) -> Vec<&str> {
        self.0.keys().map(|v| v.as_str()).collect::<Vec<&str>>()
    }
}
