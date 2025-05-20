use std::collections::HashSet;
use std::io::Cursor;
use rocksdb::{ColumnFamilyRef, Direction, IteratorMode, WriteBatch, DB};
use uuid::Uuid;
use crate::models::basics::components::Component;
use crate::models::basics::identifier::Identifier;
use crate::models::basics::json::Json;
use crate::util;
use anyhow::Result;
use crate::storage::rocksdb::managers::vertex::vertex_property_value::VertexPropertyValueManager;


// key: | vertex_id (uuid) | property_name (identifier) |
// value: | property_value (json) |

pub type OwnedPropertyItem = (Uuid, Identifier, Json);

pub(crate) struct VertexPropertyManager<'a> {
    db: &'a DB,
    cf: ColumnFamilyRef<'a>,
}

impl<'a> VertexPropertyManager<'a> {
    pub fn new(db: &'a DB) -> Self {
        VertexPropertyManager {
            db,
            cf: db.cf_handle("vertex_properties:v2").unwrap(),
        }
    }

    fn key(&self, vertex_id: Uuid, name: Identifier) -> Vec<u8> {
        util::serialize(&[
            Component::Uuid(vertex_id),
            Component::FixedLengthString(&name.0),
        ])
    }

    pub fn iterate_for_owner(&self, vertex_id: Uuid) -> Result<impl Iterator<Item = Result<OwnedPropertyItem>>> {
        let prefix = util::serialize(&[Component::Uuid(vertex_id)]);
        let iterator = self.db
            .iterator_cf(&self.cf, IteratorMode::From(&prefix, Direction::Forward));

        let filtered = util::take_with_prefix(iterator, prefix);

        Ok(filtered.map(move |item| -> Result<OwnedPropertyItem> {
            let (k, v) = item?;
            let mut cursor = Cursor::new(k);
            let owner_id = util::Deserializer::deserialize_uuid(&mut cursor)?;
            debug_assert_eq!(vertex_id, owner_id);
            let name_str = util::Deserializer::read_fixed_length_string(&mut cursor)?;
            let name = unsafe { Identifier::new_unchecked(name_str) };
            let value = serde_json::from_slice(&v)?;
            Ok((owner_id, name, value))
        }))
    }

    pub fn get(&self, vertex_id: Uuid, name: Identifier) -> Result<Option<Json>> {
        match self.db.get_cf(&self.cf, self.key(vertex_id, name))? {
            Some(value_bytes) => Ok(Some(serde_json::from_slice(&value_bytes)?)),
            None => Ok(None),
        }
    }

    pub fn set(
        &self,
        batch: &mut WriteBatch,
        indexed_properties: &HashSet<Identifier>,
        vertex_id: Uuid,
        property_name: Identifier,
        value: &Json,
    ) -> Result<()> {
        let is_indexed = indexed_properties.contains(&property_name);
        // serialize the key
        let key = self.key(vertex_id, property_name);
        if is_indexed {
            self.delete(batch, indexed_properties, vertex_id, property_name)?;
        }
        let value_json = serde_json::to_vec(value)?;
        batch.put_cf(&self.cf, &key, &value_json);
        if is_indexed {
            let vertex_property_value_manager = VertexPropertyValueManager::new(self.db);
            vertex_property_value_manager.set(batch, vertex_id, property_name, value);
        }
        Ok(())
    }

    pub fn delete(
        &self,
        batch: &mut WriteBatch,
        indexed_properties: &HashSet<Identifier>,
        vertex_id: Uuid,
        name: Identifier,
    ) -> Result<()> {
        if indexed_properties.contains(&name) {
            if let Some(value) = self.get(vertex_id, name)? {
                let vertex_property_value_manager = VertexPropertyValueManager::new(self.db);
                vertex_property_value_manager.delete(batch, vertex_id, name, &value);
            }
        }
        batch.delete_cf(&self.cf, self.key(vertex_id, name));
        Ok(())
    }
}