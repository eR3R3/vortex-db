use serde::{Deserialize, Serialize};
use crate::models::compounds::cond::Cond;
use crate::models::compounds::field::Fields;
use crate::models::compounds::range::IdRange;
use crate::models::compounds::table::Table;
use crate::models::statement::group::Groups;
use crate::models::statement::idiom::Idiom;
use crate::models::statement::limit::Limit;
use crate::models::statement::order::Ordering;
use crate::models::statement::split::Splits;
use crate::models::statement::start::Start;

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub struct Graph {
    pub dir: Dir,
    pub expr: Option<Fields>,
    pub what: GraphSubjects,
    pub cond: Option<Cond>,
    pub split: Option<Splits>,
    pub group: Option<Groups>,
    pub order: Option<Ordering>,
    pub limit: Option<Limit>,
    pub start: Option<Start>,
    pub alias: Option<Idiom>,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub enum Dir {
    /// `<-`
    In,
    /// `->`
    Out,
    /// `<->`
    Both,
}

impl Default for Dir {
    fn default() -> Self {
        Self::Both
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub struct GraphSubjects(pub Vec<GraphSubject>);

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub enum GraphSubject {
    Table(Table),
    Range(Table, IdRange),
}

