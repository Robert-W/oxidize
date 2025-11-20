use axum::{
    extract::MatchedPath,
    http::{HeaderMap, header},
};
use opentelemetry_http::{Bytes, HeaderExtractor};
use opentelemetry_semantic_conventions::{
    attribute::OTEL_STATUS_CODE,
    trace::{
        ERROR_TYPE, HTTP_REQUEST_METHOD, HTTP_RESPONSE_STATUS_CODE, HTTP_ROUTE, URL_FULL, URL_PATH,
        USER_AGENT_ORIGINAL,
    },
};
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::{MakeSpan, OnBodyChunk, OnResponse, TraceLayer},
};
use tracing::field::Empty;
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::constants::{HTTP_REQUEST_HEADER_HOST, HTTP_RESPONSE_BODY_SIZE};

// Construct the tracing layer that will customize spans created for http requests
pub fn new_trace_layer() -> TraceLayer<
    SharedClassifier<ServerErrorsAsFailures>,
    OtelMakeSpan,
    (),
    OtelOnResponse,
    OtelOnBodyChunk,
    (),
    (),
> {
    TraceLayer::new_for_http()
        .make_span_with(OtelMakeSpan)
        .on_request(())
        .on_response(OtelOnResponse)
        .on_body_chunk(OtelOnBodyChunk)
        .on_failure(())
        .on_eos(())
}

#[derive(Clone)]
pub struct OtelMakeSpan;

impl<T> MakeSpan<T> for OtelMakeSpan {
    fn make_span(&mut self, request: &axum::http::Request<T>) -> tracing::Span {
        // Pull the span from the request data, or default to an unsampled context
        let context = get_remote_context(request.headers());

        let user_agent = request
            .headers()
            .get(header::USER_AGENT)
            .map_or("", |v| v.to_str().unwrap_or(""));

        let host = request
            .headers()
            .get(header::HOST)
            .map_or("", |v| v.to_str().unwrap_or(""));

        let target = request
            .extensions()
            .get::<MatchedPath>()
            .map(MatchedPath::as_str)
            .unwrap_or("");

        let span_name = format!("{} {}", request.method(), target);

        let span = tracing::trace_span!(
            "http_request",
            otel.name = span_name,
            otel.kind = "server",
            { ERROR_TYPE } = Empty,
            { HTTP_REQUEST_METHOD } = %request.method(),
            { HTTP_REQUEST_HEADER_HOST } = host,
            { HTTP_RESPONSE_BODY_SIZE } = Empty,
            { HTTP_RESPONSE_STATUS_CODE } = Empty,
            { HTTP_ROUTE } = target,
            { OTEL_STATUS_CODE } = Empty,
            { URL_FULL } = %request.uri(),
            { URL_PATH } = request.uri().path(),
            { USER_AGENT_ORIGINAL } = user_agent,
        );

        let _ = span.set_parent(context);

        span
    }
}

fn get_remote_context(headers: &HeaderMap) -> opentelemetry::Context {
    opentelemetry::global::get_text_map_propagator(|propagator| {
        propagator.extract(&HeaderExtractor(headers))
    })
}

#[derive(Clone)]
pub struct OtelOnResponse;

impl<T> OnResponse<T> for OtelOnResponse {
    fn on_response(
        self,
        response: &opentelemetry_http::Response<T>,
        _latency: std::time::Duration,
        span: &tracing::Span,
    ) {
        let status = response.status();
        span.record(HTTP_RESPONSE_STATUS_CODE, status.as_u16());
    }
}

#[derive(Clone)]
pub struct OtelOnBodyChunk;

impl OnBodyChunk<Bytes> for OtelOnBodyChunk {
    fn on_body_chunk(&mut self, chunk: &Bytes, _latency: std::time::Duration, span: &tracing::Span) {
        span.record(HTTP_RESPONSE_BODY_SIZE, chunk.len());
    }
}
