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

#[derive(Debug, Clone)]
pub enum Joint {
    Floating,
    Prismatic,
    Revolute(Revolute),
    Spherical,
}

#[derive(Debug, Clone)]
pub struct Connection {
    inner_body: Option<Body>,
    outer_body: Option<Body>,    
}
impl Default for Connection {
    fn default() -> Self {
        Self {
            inner_body: None,
            outer_body: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Revolute {
    name: String,
    connection: Connection,
    node: Uuid,
}

impl Revolute {
    pub fn new(name: String, id: Uuid) -> Self {
        Self {
            name: name,
            connection: Connection::default(),
            node: id,
        }
    }
}
