#[cfg(feature = "schema")]
use schemars::JsonSchema;
use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonNewThreshold {
    pub branch:  Uuid,
    pub testbed: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct JsonThreshold {
    pub uuid:    Uuid,
    pub branch:  Uuid,
    pub testbed: Uuid,
    pub z_score: Option<Uuid>,
    pub t_test:  Option<Uuid>,
}