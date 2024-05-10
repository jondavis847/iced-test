#[derive(Debug, Clone)]
pub enum Modals {
    Base,
    Body(BodyModal),
    Revolute(RevoluteModal),
}

#[derive(Debug, Clone)]
pub struct BodyModal {
    pub name: String,
}

impl BodyModal {
    pub fn new(name: String) -> Self {
        Self{name}
    }
}

#[derive(Debug, Clone)]
pub struct RevoluteModal {
    pub name: String,
}

impl RevoluteModal {
    pub fn new(name: String) -> Self {
        Self{name}
    }
}
