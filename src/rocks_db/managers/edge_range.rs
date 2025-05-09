use rocksdb::{ColumnFamilyRef, WriteBatch, DB};
use crate::models::compounds::edge::Edge;
use anyhow::Result;
use crate::models::basics::components::Component;
use crate::util;

pub(crate) struct EdgeRangeManager<'a> {
    db: &'a DB,
    cf: ColumnFamilyRef<'a>,
}

impl<'a> EdgeRangeManager<'a> {
    pub fn new(db: &'a DB) -> Self {
        EdgeRangeManager {
            db,
            cf: db.cf_handle("edge_ranges:v2").unwrap(),
        }
    }

    pub fn new_reversed(db: &'a DB) -> Self {
        EdgeRangeManager {
            db,
            cf: db.cf_handle("reversed_edge_ranges:v2").unwrap(),
        }
    }

    fn key(&self, edge: &Edge) -> Vec<u8> {
        util::serialize(&[
            Component::Uuid(edge.outbound_id),
            Component::Identifier(edge.kind),
            Component::Uuid(edge.inbound_id),
        ])
    }

    pub fn set(&self, batch: &mut WriteBatch, edge: &Edge) -> Result<()> {
        let key = self.key(edge);
        batch.put_cf(&self.cf, &key, []);
        Ok(())
    }

    pub fn contains(&self, edge: &Edge) -> Result<bool> {
        Ok(self.db.get_cf(&self.cf, self.key(edge))?.is_some())
    }

    pub fn delete(&self, batch: &mut WriteBatch, edge: &Edge) -> Result<()> {
        batch.delete_cf(&self.cf, self.key(edge));
        Ok(())
    }
}