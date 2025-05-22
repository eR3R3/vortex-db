use serde::{Deserialize, Serialize};
use crate::models::compounds::value::Value;
use crate::models::statement::idiom::Idiom;

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub struct Fields(pub Vec<Field>, pub bool);

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub enum Field {
    /// The `*` in `SELECT * FROM ...`
    #[default]
    All,
    /// The 'rating' in `SELECT rating FROM ...`
    Single {
        expr: Value,
        /// The `quality` in `SELECT rating AS quality FROM ...`
        alias: Option<Idiom>,
    },
}