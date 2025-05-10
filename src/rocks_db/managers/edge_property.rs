use rocksdb::{ColumnFamilyRef, DB};

pub(crate) struct EdgePropertyManager<'a> {
    db: &'a DB,
    cf: ColumnFamilyRef<'a>,
}

impl<'a> EdgePropertyManager<'a> {
    pub fn new(db: &'a DB) -> Self {
        EdgePropertyManager {
            db,
            cf: db.cf_handle("edge_properties:v2").unwrap(),
        }
    }

    
}