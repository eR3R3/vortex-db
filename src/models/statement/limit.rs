use serde::{Deserialize, Serialize};
use crate::models::compounds::value::Value;

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub struct Limit(pub Value);