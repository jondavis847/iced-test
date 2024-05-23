use super::MultibodyTrait;
use uuid::Uuid;

pub mod revolute;
use revolute::Revolute;
use crate::ui::dummies::DummyComponent;

#[derive(Debug, Clone, Copy)]
pub enum Joint {
    //Floating,
    //Prismatic,
    Revolute(Revolute),
    //Spherical,
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
            Joint::Revolute(joint) => joint.inherit_from(dummy)
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
