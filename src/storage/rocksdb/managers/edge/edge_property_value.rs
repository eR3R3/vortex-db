use rocksdb::{ColumnFamilyRef, DB};

// key: | property_name (identifier) | property_value (json) | outbound_id (uuid) | edge_kind (identifier) | inbound_id (uuid) |
// value: it has no value

pub(crate) struct EdgePropertyValueManager<'a> {
    db: &'a DB,
    cf: ColumnFamilyRef<'a>,
}

