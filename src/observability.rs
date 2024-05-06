pub(crate) mod otel;

use std::{env, process};

use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::{runtime, trace, Resource};
use tracing_subscriber::layer::Layer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::{prelude::*, Registry};

/// Initialize logging and tracing here. The logging layer should use the
/// normal EnvFilter (e.g. RUST_LOG), use something else for tracing. Ideally
/// we want logs in one stream and traces in another
pub(crate) fn init() {
    let mut layers: Vec<Box<dyn Layer<Registry> + Send + Sync>> = Vec::new();

    // Add any/all layers here, good place to conditionally add layers based
    // on environment or presence of variables/flags
    layers.push(create_logging_layer());
    layers.push(create_tracing_layer());

    let subscribers = tracing_subscriber::registry::Registry::default().with(layers);
    tracing::subscriber::set_global_default(subscribers).unwrap();

    // Set the TraceContextPropagator, this allows us to fetch context from
    // headers later in our agplication, which we use to enable distributed
    // tracing
    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());
}

fn create_logging_layer() -> Box<dyn Layer<Registry> + Send + Sync> {
    tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(true)
        .pretty()
        .with_filter(EnvFilter::from_default_env())
        .boxed()
}

fn create_tracing_layer() -> Box<dyn Layer<Registry> + Send + Sync> {
    let collector_endpoint =
        env::var("COLLECTOR_ENDPOINT").expect("Missing COLLECTOR_ENCPOINT environment variable");
    let filter: EnvFilter = "oxidize=TRACE".parse().unwrap();
    let pid = process::id().to_string();
    let service_name = "oxidize";

    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(collector_endpoint);

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_trace_config(trace::Config::default().with_resource(Resource::new(vec![
            KeyValue::new("service.name", service_name),
            KeyValue::new("process.pid", pid),
        ])))
        .with_exporter(exporter)
        .install_batch(runtime::Tokio)
        .unwrap();

    tracing_opentelemetry::layer()
        .with_tracer(tracer)
        .with_filter(filter)
        .boxed()
}
