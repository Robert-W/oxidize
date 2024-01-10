/// This module is meant as an example of how to have a separate response struct
/// from the model struct. This is not always necessary, and in the case of the
/// Sample struct, it is not. You could move the `impl Responder` to the Sample
/// struct and this would still work fine. This pattern is more common when you
/// don't want to return all the values in your struct, or you need to filter
/// some of them based on permissions. Use whatever pattern suits your need.
use actix_web::{Responder, body::BoxBody, HttpResponse, http::header::ContentType};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::model::Sample;

#[derive(Deserialize, Serialize, Debug)]
pub struct SampleResponse {
    pub id: Uuid,
    pub name: String,
    pub created: NaiveDateTime,
    pub last_updated: NaiveDateTime,
}

impl From<Sample> for SampleResponse {
    fn from(sample: Sample) -> Self {
        Self {
            id: sample.id,
            name: sample.name,
            created: sample.created,
            last_updated: sample.last_updated,
        }
    }
}

impl Responder for SampleResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
