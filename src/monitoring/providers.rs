use opentelemetry::{propagation::TextMapCompositePropagator, trace::TracerProvider};
use opentelemetry_otlp::{Compression, SpanExporter, WithExportConfig, WithHttpConfig};
use opentelemetry_sdk::{
    propagation::{BaggagePropagator, TraceContextPropagator},
    trace::{BatchConfigBuilder, BatchSpanProcessor, SdkTracerProvider},
    Resource,
};
use tracing_subscriber::{prelude::*, EnvFilter, Layer, Registry};

use crate::{constants, monitoring::tags::get_default_tags};

#[derive(Clone, Debug)]
pub struct Providers {
    pub tracer: SdkTracerProvider,
}

impl Providers {
    pub async fn init() -> Self {
        // Enable distributed tracing by propogating baggage and trace context
        let baggage_propagator = BaggagePropagator::new();
        let context_propagator = TraceContextPropagator::new();
        let composite_propagator = TextMapCompositePropagator::new(vec![
            Box::new(baggage_propagator),
            Box::new(context_propagator),
        ]);

        opentelemetry::global::set_text_map_propagator(composite_propagator);

        // Create a tracing layer/provider if we have a collector endpoint
        let collector_url = std::env::var("COLLECTOR_URL").expect("COLLECTOR_URL is required");
        let tracing_provider = create_tracing_provider(collector_url);

        // Create a logging layer to connect logging to tracing macros
        let layers = vec![
            create_logging_layer(),
            create_tracing_layer(tracing_provider.clone()),
        ];

        tracing_subscriber::registry().with(layers).init();

        Self {
            tracer: tracing_provider,
        }
    }

    pub fn shutdown(&self) {
        let _ = &self
            .tracer
            .force_flush()
            .unwrap_or_else(|err| eprintln!("Failed to flush traces before exit: {err}"));

        let _ = &self
            .tracer
            .shutdown()
            .unwrap_or_else(|err| eprintln!("Failed to shutdown the trace provider: {err}"));
    }
}

fn create_logging_layer() -> Box<dyn Layer<Registry> + Send + Sync> {
    tracing_subscriber::fmt::layer()
        .with_line_number(true)
        .with_file(true)
        .json()
        .with_filter(EnvFilter::from_default_env())
        .boxed()
}

fn create_tracing_provider(collector_url: String) -> SdkTracerProvider {
    let exporter = SpanExporter::builder()
        .with_http()
        .with_compression(Compression::Zstd)
        .with_endpoint(collector_url)
        .build()
        .expect("Unable to build SpanExporter");

    let batch_config = BatchConfigBuilder::default().build();
    let batch_processor = BatchSpanProcessor::builder(exporter)
        .with_batch_config(batch_config)
        .build();

    let resource = Resource::builder()
        .with_attributes(get_default_tags())
        .build();

    let provider = SdkTracerProvider::builder()
        .with_span_processor(batch_processor)
        .with_resource(resource)
        .build();

    provider
}

fn create_tracing_layer(provider: SdkTracerProvider) -> Box<dyn Layer<Registry> + Send + Sync> {
    let filter = format!("{}={}", constants::SERVICE_NAME, "TRACE")
        .parse::<EnvFilter>()
        .unwrap();

    tracing_opentelemetry::layer()
        .with_tracer(provider.tracer(constants::SERVICE_NAME))
        .with_filter(filter)
        .boxed()
}
