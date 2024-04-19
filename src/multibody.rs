#[derive(Default, Debug)]
pub struct Body {
    pub name: String,
}

impl Body {
    pub fn new(name: String) -> Self {
        Body { name }
    }
}

enum Joint {
    Floating,
    Prismatic,
    Revolute,
    Spherical,
}
