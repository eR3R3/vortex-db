use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{Cursor, Write};
use byteorder::{BigEndian, WriteBytesExt};
use uuid::Uuid;
use crate::models::basics::identifier::Identifier;
use crate::models::basics::json::Json;
use anyhow::Result;

pub enum Component<'a> {
    /// A UUID.
    Uuid(Uuid),
    /// A fixed length string.
    FixedLengthString(&'a str),
    /// An identifier.
    Identifier(Identifier),
    /// A JSON value.
    Json(&'a Json),
}

impl<'a> Component<'a> {
    pub fn byte_len(&self) -> usize {
        match *self {
            Component::Uuid(_) => 16,
            Component::FixedLengthString(s) => s.len(),
            Component::Identifier(t) => t.0.len() + 1,
            Component::Json(_) => 8,
        }
    }

    pub fn writes(&self, cursor: &mut Cursor<Vec<u8>>) -> Result<()> {
        match *self {
            Component::Uuid(id) => Ok(cursor.write_all(id.as_bytes())?),
            Component::FixedLengthString(str) => Ok(cursor.write_all(str.as_bytes())?),
            Component::Identifier(identifier) => {
                // the length of identifier must be under u8
                // the .len() for String returns the number of bytes
                cursor.write_all(&[identifier.0.len() as u8])?;
                Ok(cursor.write_all(identifier.0.as_bytes())?)
            }
            Component::Json(json) => {
                let mut hasher = DefaultHasher::new();
                json.hash(&mut hasher);
                let hash = hasher.finish();
                cursor.write_u64::<BigEndian>(hash)?;
                Ok(())
            }
        }
    }
}