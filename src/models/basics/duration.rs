use std::time;
use serde::{Deserialize, Serialize};


#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash, Ord, )]
#[non_exhaustive]
pub struct Duration(pub time::Duration);