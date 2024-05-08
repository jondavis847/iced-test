#[derive(Debug, Clone)]
pub enum Modals {
    Base,
    Body(BodyModal),
    Revolute,
}

#[derive(Debug, Clone)]
pub struct BodyModal {
    pub name: String,
}

impl<'a> BodyModal {
    pub fn new(name: String) -> Self {
        Self{name}
    }
}
