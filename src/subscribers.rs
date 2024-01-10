use tracing::Subscriber;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

// Currently only uses a default layer. We can configure multiple layers if we
// wish so we could write to stdout, and honeycomb or a file for example.
pub fn all() -> impl Subscriber {
    // Optional way to construct an env filter here for this specific layer
    // let filter: EnvFilter = "oxidize=trace".parse().unwrap();
    // Construct our default layer with the env filter
    let default_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .with_file(true)
        .pretty()
        .with_filter(EnvFilter::from_default_env())
        .boxed();

    tracing_subscriber::registry::Registry::default().with(default_layer)
}
