/// This module is meant as an example of how to have a separate response struct
/// from the model struct. This is not always necessary, and in the case of the
/// Sample struct, it is not.  This pattern is more common when you don't want
/// to return all the values in your struct, or you need to filter some of them
/// based on permissions. Use whatever pattern suits your need.
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
