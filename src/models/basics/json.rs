use std::sync::Arc;

#[derive(Clone, Eq, Debug, PartialEq, Hash)]
pub struct Json(pub Arc<serde_json::Value>);