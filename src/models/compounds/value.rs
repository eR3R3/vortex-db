use std::collections::BTreeMap;
use std::time::Duration;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::basics::number::Number;
use crate::models::compounds::id::Id;


#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub enum Value {
    #[default]
    Null,
    Bool(bool),
    Number(Number),       // You can define as f64 or a simple enum of i64/f64
    String(String),       // Use String instead of Strand wrapper
    Datetime(DateTime<Utc>),
    Duration(Duration),
    Uuid(Uuid),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
    Thing(String, Id),         // Refers to another document
}