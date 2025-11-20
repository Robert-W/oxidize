use opentelemetry::KeyValue;
use opentelemetry_semantic_conventions::resource;

use crate::constants;

pub fn get_default_tags() -> Vec<KeyValue> {
    vec![KeyValue::new(
        resource::SERVICE_NAME,
        constants::SERVICE_NAME,
    )]
}

