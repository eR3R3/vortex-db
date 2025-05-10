use std::sync::Arc;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Eq, Debug, PartialEq, Hash)]
pub struct Json(pub Arc<serde_json::Value>);

impl Json {
    /// Constructs a new JSON type.
    ///
    /// # Arguments
    /// * `value`: The JSON value.
    pub fn new(value: serde_json::Value) -> Self {
        Self(Arc::new(value))
    }
}


impl Serialize for Json {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self.0).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Json {
    fn deserialize<D>(deserializer: D) -> Result<Json, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v: serde_json::Value = Deserialize::deserialize(deserializer)?;
        Ok(Json::new(v))
    }
}