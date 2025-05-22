use serde::{Deserialize, Serialize};
use crate::models::compounds::value::Value;

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub struct Permissions {
    pub select: Permission,
    pub create: Permission,
    pub update: Permission,
    pub delete: Permission,
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub enum Permission {
    None,
    #[default]
    Full,
    Specific(Value),
}