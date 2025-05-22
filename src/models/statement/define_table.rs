use crate::models::statement::permission::Permissions;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub struct DefineTableStatement {
    pub id: Option<u32>,
    pub name: String,
    pub drop: bool,
    pub full: bool,
    pub permissions: Permissions,
    pub comment: Option<String>,
    /// Should we overwrite the field definition if it already exists
    pub overwrite: bool,
}