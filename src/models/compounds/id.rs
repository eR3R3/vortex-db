use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::basics::number::Number;
use crate::models::compounds::value::Value;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub enum Id {
    Uuid(Uuid),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
    Generate(Gen),
    Number(Number),       // You can define as f64 or a simple enum of i64/f64
    String(String),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Hash)]
pub enum Gen {
    Rand,
    Uuid,
}