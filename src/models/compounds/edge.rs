use uuid::Uuid;
use crate::models::basics::identifier::Identifier;

#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    pub outbound_id: Uuid,
    pub kind: Identifier,
    pub inbound_id: Uuid,
}

impl Edge {
    pub fn new(outbound_id: Uuid, kind: Identifier, inbound_id: Uuid) -> Self {
        Edge {
            inbound_id,
            outbound_id,
            kind
        }
    }

    pub fn reverse(&self) -> Edge {
        Edge::new(self.inbound_id, self.kind, self.outbound_id)
    }
}