use rocksdb::{WriteBatch, DB};
use anyhow::Result;
use crate::models::compounds::edge::Edge;
use crate::rocks_db::managers::edge_range::EdgeRangeManager;

pub struct EdgeManager<'a> {
    db: &'a DB,
}

impl<'a> EdgeManager<'a> {
    pub fn new(db: &'a DB) -> Self {
        EdgeManager { db }
    }

    pub fn set(&self, batch: &mut WriteBatch, edge: &Edge) -> Result<()> {
        let edge_range_manager = EdgeRangeManager::new(self.db);
        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db);
        let reversed_edge = edge.reverse();

        // check if the manager already contains the edge we want to add
        if edge_range_manager.contains(edge)? {
            // if yes delete the original edge and its reversed edge
            edge_range_manager.delete(batch, edge)?;
            reversed_edge_range_manager.delete(batch, &reversed_edge)?;
        }

        // reset the edges
        edge_range_manager.set(batch, edge)?;
        reversed_edge_range_manager.set(batch, &reversed_edge)?;
        Ok(())
    }
}