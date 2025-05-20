use std::collections::HashSet;
use rocksdb::{WriteBatch, DB};
use anyhow::Result;
use crate::models::basics::identifier::Identifier;
use crate::models::compounds::edge::Edge;
use crate::storage::rocksdb::managers::edge::edge_property::EdgePropertyManager;
use crate::storage::rocksdb::managers::edge::edge_range::EdgeRangeManager;

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

    pub fn delete(
        &self,
        batch: &mut WriteBatch,
        indexed_properties: &HashSet<Identifier>,
        edge: &Edge,
    ) -> Result<()> {
        let edge_range_manager = EdgeRangeManager::new(self.db);
        edge_range_manager.delete(batch, edge)?;

        let reversed_edge_range_manager = EdgeRangeManager::new_reversed(self.db);
        reversed_edge_range_manager.delete(batch, &edge.reverse())?;

        let edge_property_manager = EdgePropertyManager::new(self.db);
        for item in edge_property_manager.iterate_for_owner(edge)? {
            let (edge_property_edge, edge_property_name, _) = item?;
            edge_property_manager.delete(batch, indexed_properties, &edge_property_edge, edge_property_name)?;
        }
        Ok(())
    }
}