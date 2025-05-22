use serde::{Deserialize, Serialize};
use crate::models::statement::part::Part;

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub struct Idiom(pub Vec<Part>);