// pub mod common;
//
// use actix_web::{dev::ServiceResponse, test, web, App};
// use common::get_pool;
// use oxidize::endpoints;
// use serde_json::{json, Value};
// use uuid::Uuid;
//
// async fn post_sample(name: &str) -> ServiceResponse {
//     let app = test::init_service(create_server!(get_pool().await)).await;
//     let req = test::TestRequest::post()
//         .set_form(json!({ "id": Uuid::new_v4(), "name": name }))
//         .uri("/api/sample/")
//         .to_request();
//
//     test::call_service(&app, req).await
// }
//
// #[test]
// async fn create_sample() {
//     let res = post_sample("scrappy").await;
//
//     assert_eq!(res.status(), 200);
// }
//
// #[test]
// async fn get_sample() {
//     let app = test::init_service(create_server!(get_pool().await)).await;
//     let id = "174db1e0-6f40-4a2c-a015-13ca37eb193d";
//     let name = "scooby-doo";
//     let req = test::TestRequest::get()
//         .uri(&format!("/api/sample/{}", &id))
//         .to_request();
//     let res = test::call_service(&app, req).await;
//     assert_eq!(res.status(), 200);
//
//     let sample: Value = test::read_body_json(res).await;
//     assert_eq!(sample["name"], name);
// }
//
// #[test]
// async fn list_samples() {
//     let app = test::init_service(create_server!(get_pool().await)).await;
//     let req = test::TestRequest::get().uri("/api/sample/").to_request();
//
//     let res = test::call_service(&app, req).await;
//     assert_eq!(res.status(), 200);
//
//     let samples: Vec<Value> = test::read_body_json(res).await;
//     assert!(samples.len() >= 5);
// }
//
// #[test]
// async fn update_sample() {
//     let app = test::init_service(create_server!(get_pool().await)).await;
//     let sample: Value = test::read_body_json(post_sample("jensen").await).await;
//     let updates = json!({ "name": "dean" });
//
//     let req = test::TestRequest::put()
//         .set_form(updates)
//         // Annoying but to get a string from Value, you have to as_str it. If
//         // you don't, you end up with quotes (e.g. /api/sample/"id")
//         .uri(&format!("/api/sample/{}", sample["id"].as_str().unwrap()))
//         .to_request();
//
//
//     let res = test::call_service(&app, req).await;
//     assert_eq!(res.status(), 200);
//
//     let updated: Value = test::read_body_json(res).await;
//     assert_eq!(updated["id"], sample["id"]);
//     assert_ne!(updated["name"], sample["name"]);
//     assert_ne!(updated["last_updated"], sample["last_updated"]);
// }
//
//
// #[test]
// async fn delete_sample() {
//     let app = test::init_service(create_server!(get_pool().await)).await;
//     let sample: Value = test::read_body_json(post_sample("sam").await).await;
//     let id = sample["id"].as_str().unwrap();
//
//     let req = test::TestRequest::delete()
//         .uri(&format!("/api/sample/{}", id))
//         .to_request();
//
//     let res = test::call_service(&app, req).await;
//     assert_eq!(res.status(), 204);
//
//     // Verify and do a get on this id, should get a 404
//     let req = test::TestRequest::get()
//         .uri(&format!("/api/sample/{}", id))
//         .to_request();
//
//     let res = test::call_service(&app, req).await;
//     assert_eq!(res.status(), 404);
// }
