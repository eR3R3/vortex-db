use std::collections::HashMap;
use crate::models::statement::mutations::TableMutations;

#[non_exhaustive]
pub struct Writer {
    buf: Buffer,
}

#[non_exhaustive]
pub struct Buffer {
    pub b: HashMap<ChangeKey, TableMutations>,
}

#[derive(Hash, Eq, PartialEq, Debug)]
#[non_exhaustive]
pub struct ChangeKey {
    pub ns: String,
    pub db: String,
    pub tb: String,
}
