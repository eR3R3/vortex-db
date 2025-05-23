use std::io::{Cursor, Read};
use std::ops::Range;
use anyhow::Result;
use rocksdb::DBIterator;
use uuid::Uuid;
use crate::config::Key;
use crate::models::basics::components::Component;
use crate::models::basics::identifier;
use crate::models::basics::identifier::Identifier;


pub(crate) fn take_with_prefix(iterator: DBIterator, prefix: Vec<u8>) -> impl Iterator<Item = Result<(Box<[u8]>, Box<[u8]>)>> {
    iterator.take_while(move |item| -> bool {
        if let Ok((ref k, _)) = *item {
            k.starts_with(&prefix)
        } else { true }})
        .map(|res| res.map_err(Into::into))
}

pub fn serialize(components: &[Component]) -> Vec<u8> {
    let len = components.iter().fold(0, |len, component| len + component.byte_len());
    let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::with_capacity(len));
    for component in components {
        component
            .writes(&mut cursor)
            .expect("failed to write bytes to in-memory buffer");
    }

    cursor.into_inner()
}

pub struct Deserializer {}

impl Deserializer {
    pub unsafe fn deserialize_identifier<T: AsRef<[u8]>>(cursor: &mut Cursor<T>) -> Result<Identifier> {
        let t_len = {
            let mut buf: [u8; 1] = [0; 1];
            cursor.read_exact(&mut buf)?;
            buf[0] as usize
        };

        let mut buf = vec![0u8; t_len];
        cursor.read_exact(&mut buf)?;
        let s = str::from_utf8_unchecked(&buf).to_string();
        Ok(Identifier::new_unchecked(s))
    }

    pub fn deserialize_uuid<T: AsRef<[u8]>>(cursor: &mut Cursor<T>) -> Result<Uuid> {
        let mut buffer = [0u8, 16];
        cursor.read_exact(&mut buffer)?;
        Ok(Uuid::from_slice(&buffer)?)
    }

    pub fn read_fixed_length_string<T: AsRef<[u8]>>(cursor: &mut Cursor<T>) -> Result<String> {
        let mut buf = String::new();
        cursor.read_to_string(&mut buf)?;
        Ok(buf)
    }
}

pub fn convert_to_cursor<T: AsRef<[u8]>>(inner: T) -> Cursor<T> {
    Cursor::new(inner)
}

#[derive(Debug)]
pub struct Batch<T> {
    pub next: Option<Range<Key>>,
    pub result: Vec<T>,
}

impl<T> Batch<T> {
    /// Create a new batch scan result.
    pub fn new(next: Option<Range<Key>>, result: Vec<T>) -> Self {
        Self {
            next,
            result,
        }
    }
}

pub fn advance_key(key: &mut [u8]) {
    for b in key.iter_mut().rev() {
        *b = b.wrapping_add(1);
        if *b != 0 {
            break;
        }
    }
}