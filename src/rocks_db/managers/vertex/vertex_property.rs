use std::io::Cursor;
use rocksdb::{ColumnFamilyRef, Direction, IteratorMode, DB};
use uuid::Uuid;
use crate::models::basics::components::Component;
use crate::models::basics::identifier::Identifier;
use crate::models::basics::json::Json;
use crate::util;
use anyhow::Result;

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
        let prefix = util::serialize(&[util::Component::Uuid(vertex_id)]);
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
}