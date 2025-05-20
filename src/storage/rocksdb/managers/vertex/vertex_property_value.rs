use rocksdb::{ColumnFamilyRef, WriteBatch, DB};
use uuid::Uuid;
use crate::models::basics::components::Component;
use crate::models::basics::identifier::Identifier;
use crate::models::basics::json::Json;
use crate::util;

// key: | property_name (identifier) | property_value (json) | vertex_id (uuid) |
// value: it has no value

pub(crate) struct VertexPropertyValueManager<'a> {
    db: &'a DB,
    cf: ColumnFamilyRef<'a>,
}

impl<'a> VertexPropertyValueManager<'a> {
    pub fn new(db: &'a DB) -> Self {
        VertexPropertyValueManager {
            db,
            cf: db.cf_handle("vertex_property_values:v2").unwrap(),
        }
    }

    fn key(&self, property_name: Identifier, property_value: &Json, vertex_id: Uuid) -> Vec<u8> {
        util::serialize(&[
            Component::Identifier(property_name),
            Component::Json(property_value),
            Component::Uuid(vertex_id),
        ])
    }

    pub(crate) fn set(&self, p0: &mut WriteBatch, p1: Uuid, p2: Identifier, p3: &Json) {
        todo!()
    }

    pub(crate) fn delete(&self, p0: &mut WriteBatch, p1: Uuid, p2: Identifier, p3: &Json) {
        todo!()
    }
}