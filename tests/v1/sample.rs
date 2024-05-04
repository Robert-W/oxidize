#[cfg(test)]
mod sample_tests {
    use crate::common;
    use reqwest::StatusCode;
    use serde_json::json;

    #[tokio::test]
    async fn create() {
        let params = json!({ "name": "scrappy" });
        let url = common::get_service_url();
        let client = reqwest::Client::new();

        let res = client
            .post(format!("{url}/v/1/sample"))
            .json(&params)
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let response = res.json::<serde_json::Value>().await.unwrap();
        assert_eq!(response["name"], "scrappy");
    }

    #[tokio::test]
    async fn read() {
        let id = "0ef309be-dd16-447d-84c1-ec47cd8c1a8c";
        let url = common::get_service_url();
        let client = reqwest::Client::new();

        let res = client
            .get(format!("{url}/v/1/sample/{id}"))
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let response = res.json::<serde_json::Value>().await.unwrap();
        assert_eq!(response["name"], "shaggy");
    }

    #[tokio::test]
    async fn list() {
        let url = common::get_service_url();
        let client = reqwest::Client::new();

        let res = client
            .get(format!("{url}/v/1/sample"))
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let response = res.json::<serde_json::Value>().await.unwrap();
        let results = response.as_array().unwrap();
        // 5 resources are inserted via fixtures
        assert!(results.len() >= 5);
    }

    #[tokio::test]
    async fn update() {
        let id = "0ef309be-dd16-447d-84c1-ec47cd8c1a8c";
        let params = json!({ "name": "steve" });
        let url = common::get_service_url();
        let client = reqwest::Client::new();

        let res = client
            .put(format!("{url}/v/1/sample/{id}"))
            .json(&params)
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);

        let response = res.json::<serde_json::Value>().await.unwrap();
        assert_eq!(response["name"], "steve");
    }


    #[tokio::test]
    async fn delete() {
        let id = "93ee5b24-8c2d-42e7-9ed8-6f4eca7cad9a";
        let url = common::get_service_url();
        let client = reqwest::Client::new();

        let res = client
            .delete(format!("{url}/v/1/sample/{id}"))
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::NO_CONTENT);

        // Fred should now be long gone, query to see if he actually is gone
        let res = client
            .get(format!("{url}/v/1/sample/{id}"))
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }
}
