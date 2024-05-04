use std::env;

pub fn get_service_url() -> String {
    env::var("SERVICE_URL").expect("Missing SERVICE_URL which is required for testing")
}

