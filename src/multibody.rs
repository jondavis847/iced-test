use uuid::Uuid;

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

//enum Joint {
//    Floating,
//    Prismatic,
//    Revolute,
//    Spherical,
//}
