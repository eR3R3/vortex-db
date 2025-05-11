use std::collections::HashSet;
use std::io::Cursor;
use rocksdb::{ColumnFamilyRef, DBIterator, Direction, IteratorMode, WriteBatch, DB};
use crate::models::basics::components::Component;
use crate::models::basics::identifier::Identifier;
use crate::models::compounds::edge::Edge;
use crate::util;
use anyhow::Result;
use crate::models::basics::json::Json;

// the key for this cf is | outbound_id(uuid) | edge kind (identifier) | inbound_id(uuid) | property name(identifier) |
// the value for this cf is | property value(json) |
// we can use the prefix method to filter and get the property
// the reason we do not put property name(identifier) into the value is that the key in one lsm tree(cf) should be unique

pub type EdgePropertyItem = (Edge, Identifier, Json);

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

    fn key(&self, edge: &Edge, name: Identifier) -> Vec<u8> {
        util::serialize(&[
            Component::Uuid(edge.outbound_id),
            Component::Identifier(edge.kind),
            Component::Uuid(edge.inbound_id),
            Component::FixedLengthString(&name.0),
        ])
    }

    pub fn iterate_for_owner(&self, edge: &Edge) -> Result<impl Iterator<Item = Result<EdgePropertyItem>>>{
        let prefix = util::serialize(&[
            Component::Uuid(edge.outbound_id),
            Component::Identifier(edge.kind),
            Component::Uuid(edge.inbound_id),
        ]);

        let iterator = self
            .db
            .iterator_cf(&self.cf, IteratorMode::From(&prefix, Direction::Forward));

        let filtered = util::take_with_prefix(iterator, prefix);

        Ok(filtered.map(move |item| -> Result<EdgePropertyItem> {
            let (k, v) = item?;
            let mut cursor = Cursor::new(k);

            let edge_property_out_id = util::Deserializer::deserialize_uuid(&mut cursor)?;
            debug_assert_eq!(edge_property_out_id, edge.outbound_id);

            let edge_property_kind = unsafe { util::Deserializer::deserialize_identifier(&mut cursor)? };
            debug_assert_eq!(edge_property_kind, edge.kind);

            let edge_property_in_id = util::Deserializer::deserialize_uuid(&mut cursor)?;
            debug_assert_eq!(edge_property_in_id, edge.inbound_id);

            let edge_property_name_str = util::Deserializer::read_fixed_length_string(&mut cursor)?;
            let edge_property_name = unsafe { Identifier::new_unchecked(edge_property_name_str) };

            let value = serde_json::from_slice::<Json>(&v)?;
            let edge_property_edge = Edge::new(edge_property_out_id, edge_property_kind, edge_property_in_id);
            Ok((edge_property_edge, edge_property_name, value))
        }))
    }

    pub fn get(&self, edge: &Edge, name: Identifier) -> Result<Option<Json>> {
        match self.db.get_cf(&self.cf, self.key(edge, name))? {
            Some(value_bytes) => Ok(Some(serde_json::from_slice(&value_bytes)?)),
            None => Ok(None),
        }
    }

    pub fn delete(
        &self,
        batch: &mut WriteBatch,
        indexed_properties: &HashSet<Identifier>,
        edge: &Edge,
        name: Identifier,
    ) -> Result<()> {
        if indexed_properties.contains(&name) {
            // this is for the other cf
            // if let Some(value) = self.get(edge, name)? {
            //     let edge_property_value_manager = EdgePropertyValueManager::new(self.db);
            //     edge_property_value_manager.delete(batch, edge, name, &value);
            // }
        }
        batch.delete_cf(&self.cf, self.key(edge, name));
        Ok(())
    }

    pub fn compact(&self) {
        self.db
            .compact_range_cf(&self.cf, Option::<&[u8]>::None, Option::<&[u8]>::None);
    }
}