use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CreateSample {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct UpdateSample {
    pub name: String,
}
