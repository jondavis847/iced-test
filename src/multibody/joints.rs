use super::MultibodyTrait;
use uuid::Uuid;

pub mod revolute;
use crate::ui::dummies::DummyComponent;
use revolute::Revolute;

pub trait JointTrait {
    fn connect_from(&mut self, from_id: Uuid);
    fn connect_to(&mut self, to_id: Uuid);
    fn delete_from(&mut self);
    fn delete_to(&mut self);
}

#[derive(Debug, Clone, Copy)]
pub enum Joint {
    //Floating,
    //Prismatic,
    Revolute(Revolute),
    //Spherical,
}

impl JointTrait for Joint {
    fn connect_from(&mut self, from_id: Uuid) {
        match self {
            Joint::Revolute(joint) => joint.connect_from(from_id),
        }
    }
    fn connect_to(&mut self, to_id: Uuid) {
        match self {
            Joint::Revolute(joint) => joint.connect_to(to_id),
        }
    }

    fn delete_from(&mut self) {
        match self {
            Joint::Revolute(joint) => joint.delete_from(),
        }
    }

    fn delete_to(&mut self) {
        match self {
            Joint::Revolute(joint) => joint.delete_to(),
        }
    }
}

impl MultibodyTrait for Joint {
    fn get_component_id(&self) -> &Uuid {
        match self {
            Joint::Revolute(revolute) => revolute.get_component_id(),
        }
    }

    fn get_dummy_id(&self) -> &Uuid {
        match self {
            Joint::Revolute(revolute) => revolute.get_dummy_id(),
        }
    }

    fn get_name_id(&self) -> Uuid {
        match self {
            Joint::Revolute(revolute) => revolute.get_name_id(),
        }
    }

    fn get_node_id(&self) -> &Uuid {
        match self {
            Joint::Revolute(revolute) => revolute.get_node_id(),
        }
    }

    fn inherit_from(&mut self, dummy: &DummyComponent) {
        match self {
            Joint::Revolute(joint) => joint.inherit_from(dummy),
        }
    }
    fn set_component_id(&mut self, id: &Uuid) {
        match self {
            Joint::Revolute(revolute) => revolute.set_component_id(id),
        }
    }

    fn set_name_id(&mut self, id: &Uuid) {
        match self {
            Joint::Revolute(revolute) => revolute.set_name_id(id),
        }
    }

    fn set_node_id(&mut self, id: &Uuid) {
        match self {
            Joint::Revolute(revolute) => revolute.set_node_id(id),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct JointParameters {
    pub constant_force: f64,
    pub dampening: f64,
    pub spring_constant: f64,
}

impl JointParameters {
    pub fn new(constant_force: f64, dampening: f64, spring_constant: f64) -> Self {
        Self {
            constant_force,
            dampening,
            spring_constant,
        }
    }
}
