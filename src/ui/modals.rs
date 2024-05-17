use crate::multibody::*;

#[derive(Debug, Clone)]
pub enum Modals {
    Base,
    Body(Body),
    Revolute(Revolute),
}