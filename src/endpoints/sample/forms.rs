use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateSample {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateSample {
    pub name: String,
}
