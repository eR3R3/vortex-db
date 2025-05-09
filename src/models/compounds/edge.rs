use uuid::Uuid;
use crate::models::basics::identifier::Identifier;

pub struct Edge {
    pub inbound_id: Uuid,
    pub outbound_id : Uuid,
    pub kind: Identifier,
}

impl Edge {
    pub fn new(inbound_id: Uuid, outbound_id: Uuid, kind: Identifier) -> Self {
        Edge {
            inbound_id,
            outbound_id,
            kind
        }
    }

    pub fn reverse(&self) -> Edge {
        Edge::new(self.outbound_id, self.inbound_id, self.kind)
    }
}