use std::sync::Arc;
use crate::models::compounds::table::Table;
use crate::models::compounds::thing::Thing;
use crate::models::compounds::value::Value;

pub(crate) struct Document {
    /// The record id of this document
    pub(super) id: Option<Arc<Thing>>,
    /// The table that we should generate a record id from
    pub(super) gen_table: Option<Table>,
    /// Whether this is the second iteration of the processing
    pub(super) retry: bool,
    pub(super) extras: Workable,
    pub(super) initial: CursorDoc,
    pub(super) current: CursorDoc,
    pub(super) initial_reduced: CursorDoc,
    pub(super) current_reduced: CursorDoc,
    pub(super) record_strategy: RecordStrategy,
}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub(crate) struct CursorDoc {
    pub(crate) rid: Option<Arc<Thing>>,
    pub(crate) ir: Option<Arc<IteratorRecord>>,
    pub(crate) doc: CursorValue,
}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub(crate) struct CursorValue {
    mutable: Value,
    read_only: Option<Arc<Value>>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum RecordStrategy {
    Count,
    KeysOnly,
    KeysAndValues,
}

#[derive(Debug)]
pub(crate) enum Workable {
    Normal,
    Insert(Arc<Value>),
    Relate(Thing, Thing, Option<Arc<Value>>),
}