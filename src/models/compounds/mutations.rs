use serde::{Deserialize, Serialize};
use crate::models::compounds::thing::Thing;
use crate::models::compounds::value::Value;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub struct TableMutations(pub String, pub Vec<TableMutation>);

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
#[non_exhaustive]
pub enum TableMutation {
    // Although the Value is supposed to contain a field "id" of Thing,
    // we do include it in the first field for convenience.
    Set(Thing, Value),
    Del(Thing),
    Def(DefineTableStatement),
    
    /// Includes the ID, current value (after change), changes that can be applied to get the original
    /// value
    /// Example, ("mytb:tobie", {{"note": "surreal"}}, [{"op": "add", "path": "/note", "value": "surreal"}], false)
    /// Means that we have already applied the add "/note" operation to achieve the recorded result
    SetWithDiff(Thing, Value, Vec<Operation>),
    /// Delete a record where the ID is stored, and the now-deleted value
    DelWithOriginal(Thing, Value),
}