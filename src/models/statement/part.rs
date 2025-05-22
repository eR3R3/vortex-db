use async_channel::RecvError;
use serde::{Deserialize, Serialize};
use crate::models::basics::number::Number;
use crate::models::compounds::value::Value;
use crate::models::statement::graph::Graph;
use crate::models::statement::idiom::Idiom;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub enum Part {
    All,
    Flatten,
    Last,
    First,
    Field(String),
    Index(Number),
    Where(Value),
    Graph(Graph),
    Value(Value),
    Start(Value),
    Method(String, Vec<Value>),
    Optional,
    Recurse(Recurse, Option<Idiom>, Option<RecurseInstruction>),
    Doc,
    RepeatRecurse,
}


#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub enum Recurse {
    Fixed(u32),
    Range(Option<u32>, Option<u32>),
}


#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub enum RecurseInstruction {
    Path {
        // Do we include the starting point in the paths?
        inclusive: bool,
    },
    Collect {
        // Do we include the starting point in the collection?
        inclusive: bool,
    },
    Shortest {
        // What ending node are we looking for?
        expects: Value,
        // Do we include the starting point in the collection?
        inclusive: bool,
    },
}