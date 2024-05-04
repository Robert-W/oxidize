mod common;
mod v1;

use reqwest::StatusCode;

#[tokio::test]
async fn health() {
    let url = common::get_service_url();
    let client = reqwest::Client::new();
    let res = client.get(format!("{url}/health")).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
}
