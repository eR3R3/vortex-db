use serde::{Deserialize, Serialize};
use crate::models::statement::idiom::Idiom;

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub struct Groups(pub Vec<Group>);

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub struct Group(pub Idiom);