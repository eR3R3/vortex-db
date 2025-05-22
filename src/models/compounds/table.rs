use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash, Ord)]
#[non_exhaustive]
pub struct Table(pub String);