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
    pub mass: f64,
    pub cmx: f64,
    pub cmy: f64,
    pub cmz: f64,
    pub ixx: f64,
    pub iyy: f64,
    pub izz: f64,
    pub ixy: f64,
    pub ixz: f64,
    pub iyz: f64,
}

/// Only to be used for the node_bar node
impl Default for Body {
    fn default() ->Self {
        Self {                        
            name: "".to_string(),
            node: Uuid::new_v4(),            
            mass: 1.0,
            cmx: 0.0,
            cmy: 0.0,
            cmz: 0.0,
            ixx: 1.0,
            iyy: 1.0,
            izz: 1.0,
            ixy: 0.0,
            ixz: 0.0,
            iyz: 0.0,
        }
    }
}

impl Body {
    pub fn new( name: String,
         node: Uuid,
         mass: f64,
         cmx: f64,
         cmy: f64,
         cmz: f64,
         ixx: f64,
         iyy: f64,
         izz: f64,
         ixy: f64,
         ixz: f64,
         iyz: f64,) -> Self {
        Self {name,
            node,
            mass,
            cmx,
            cmy,
            cmz,
            ixx,
            iyy,
            izz,
            ixy,
            ixz,
            iyz,}
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
    pub name: String,
    connection: Connection,
    node: Uuid,
}

/// Only to be used for the node_bar node
impl Default for Revolute {
    fn default() ->Self {
        Self {
            name: String::new(),
            connection: Connection::default(),
            node: Uuid::nil(),
        }
    }
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
