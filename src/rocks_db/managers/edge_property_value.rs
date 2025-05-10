use rocksdb::{ColumnFamilyRef, DB};

pub(crate) struct EdgePropertyValueManager<'a> {
    db: &'a DB,
    cf: ColumnFamilyRef<'a>,
}