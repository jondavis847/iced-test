use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Base {    
    pub node_id: Uuid,
}

impl Base {
    pub fn new(node_id: Uuid) -> Self {
        Self { node_id}
    }
}

#[derive(Debug, Clone)]
pub struct Body {
    pub name: String,
    pub node: Uuid,
}

impl Body {
    pub fn new(name: String, node: Uuid) -> Self {
        Body { name, node }
    }
}

pub enum Joint {
    Floating,
    Prismatic,
    Revolute,
    Spherical,
}

pub struct Connection {
    inner_body: Body,
    outer_body: Body,    
}