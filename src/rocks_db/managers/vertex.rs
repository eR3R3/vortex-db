use std::io::Cursor;
use std::ops::Deref;
use std::sync::Arc;
use rocksdb::{ColumnFamilyRef, Direction, IteratorMode, WriteBatch, DB};
use uuid::Uuid;
use crate::models::basics::components::Component;
use crate::{util};
use anyhow::Result;
use crate::models::basics::identifier::Identifier;
use crate::models::compounds::vertex::Vertex;
use crate::util::Deserializer;

pub(crate) struct VertexManager<'a> {
    db: &'a DB,
    cf: ColumnFamilyRef<'a>,
}

impl<'a> VertexManager<'a> {
    pub fn new(db: &'a DB) -> Self {
        let cf = db.cf_handle("vertices:v2").unwrap();
        VertexManager {
            db,
            cf,
        }
    }

    pub fn exists(&self, id: Uuid) -> Result<bool> {
        let raw_id = util::serialize(&[Component::Uuid(id)]);
        Ok(self.db.get_cf(self.cf.clone(), raw_id)?.is_some())
    }

    pub fn get(&self, id: Uuid) -> Result<Option<Identifier>> {
        match self.db.get_cf(self.cf.clone(), id)? {
            // if we find the target key, return the key(uuid) and the value(identifier)
            Some(value_bytes) => {
                // now we should convert this vector into identifier type
                let mut cursor = Cursor::new(value_bytes.deref());
                let identifier = unsafe { Deserializer::deserialize_identifier(&mut cursor)? };
                Ok(Some(identifier))
            }
            None => {
                Ok(None)
            }
        }
    }

    pub fn iterate_lower_bound(&self, raw_lower_bound_key: Uuid) -> impl Iterator<Item = Result<Vertex>> {
        let lower_bound_key = util::serialize(&[Component::Uuid(raw_lower_bound_key)]);
        let iter = self.db.
            iterator_cf(self.cf.clone(), IteratorMode::From(lower_bound_key.deref(), Direction::Forward));
        iter.map(|raw_pair| -> Result<Vertex> {
            let (raw_uuid, raw_identifier) = raw_pair?;
            let uuid = Deserializer::deserialize_uuid(&mut util::convert_to_cursor(raw_uuid.as_ref()))?;
            let identifier = unsafe { Deserializer
                ::deserialize_identifier(&mut util::convert_to_cursor(raw_identifier.as_ref()))? };
            Ok(Vertex::new_with_id(uuid, identifier))
        })
    }

    pub fn create(&self, batch: &mut WriteBatch, vertex: &Vertex) {
        let uuid = util::serialize(&[Component::Uuid(vertex.id)]);
        let identifier = util::serialize(&[Component::Identifier(vertex.kind)]);
        // the reason it has a put_cf function is that a single batch can cache key-value pairs that will be
        // stored in different cf
        batch.put_cf(self.cf.clone(), uuid, identifier);
    }
}