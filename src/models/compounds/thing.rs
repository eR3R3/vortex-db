use serde::{Deserialize, Serialize};
use crate::models::compounds::id::Id;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub struct Thing {
    /// Table name
    pub tb: String,
    pub id: Id,
}
