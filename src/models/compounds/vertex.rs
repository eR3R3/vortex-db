use uuid::Uuid;
use crate::models::basics::identifier::Identifier;

#[derive(Clone, Debug, PartialEq)]
pub struct Vertex {
    pub id: Uuid,
    pub kind: Identifier,
}

impl Vertex {
    pub fn new_with_id(id: Uuid, kind: Identifier) -> Vertex {
        Vertex {
            id,
            kind
        }
    }
}