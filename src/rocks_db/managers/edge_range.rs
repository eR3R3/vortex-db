use rocksdb::{ColumnFamilyRef, DB};

pub(crate) struct EdgeRangeManager<'a> {
    db: &'a DB,
    cf: ColumnFamilyRef<'a>,
}

impl<'a> EdgeRangeManager<'a> {

}