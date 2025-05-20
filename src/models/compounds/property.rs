use uuid::Uuid;
use crate::models::basics::identifier::Identifier;
use crate::models::basics::json::Json;
use crate::models::compounds::edge::Edge;
use crate::models::compounds::vertex::Vertex;


#[derive(Clone, Debug, PartialEq)]
pub struct NamedProperty {
    pub name: Identifier,
    pub value: Json,
}

impl NamedProperty {
    pub fn new(name: Identifier, value: Json) -> Self {
        Self { name, value }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct VertexProperties {
    pub vertex: Vertex,
    pub props: Vec<NamedProperty>,
}

impl VertexProperties {
    pub fn new(vertex: Vertex, props: Vec<NamedProperty>) -> Self {
        VertexProperties { vertex, props }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct VertexProperty {
    pub id: Uuid,
    pub value: Json,
}

impl VertexProperty {
    pub fn new(id: Uuid, value: Json) -> Self {
        Self { id, value }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct EdgeProperties {
    pub edge: Edge,
    pub props: Vec<NamedProperty>,
}

impl EdgeProperties {
    pub fn new(edge: Edge, props: Vec<NamedProperty>) -> Self {
        EdgeProperties { edge, props }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EdgeProperty {
    pub edge: Edge,
    pub value: Json,
}

impl EdgeProperty {
    pub fn new(edge: Edge, value: Json) -> Self {
        Self { edge, value }
    }
}