use uuid::Uuid;

pub mod base;
pub mod body;
pub mod connection;
pub mod joints;

use base::Base;
use body::Body;
use joints::{Joint, revolute::Revolute};

use crate::ui::dummies::{DummyComponent, DummyTrait};
#[derive(Debug, Clone, Copy)]
pub struct MultibodyMeta {
    component_id: Uuid,        
    dummy_id: Uuid,
    name_id: Uuid,        
    node_id: Uuid,
}

impl MultibodyMeta {
    pub fn new(component_id: Uuid, dummy_id: Uuid, name_id: Uuid, node_id: Uuid) -> Self {
        Self {
            component_id,                        
            dummy_id,
            name_id,
            node_id,
        }
    }
}

pub trait MultibodyTrait {
    fn get_component_id(&self) -> &Uuid;
    fn get_dummy_id(&self) -> &Uuid;
    fn get_name_id(&self) -> Uuid;
    fn get_node_id(&self) -> &Uuid;
    fn inherit_from(&mut self, dummy: &DummyComponent);
    fn set_component_id(&mut self, id: &Uuid);    
    fn set_name_id(&mut self, name_id: &Uuid);    
    fn set_node_id(&mut self, node_id: &Uuid);
}

#[derive(Debug, Clone, Copy)]
pub enum MultibodyComponent {
    Base(Base),
    Body(Body),
    Joint(Joint),
}

impl MultibodyTrait for MultibodyComponent {
    fn get_component_id(&self) -> &Uuid {
        match self {
            MultibodyComponent::Base(base) => base.get_component_id(),
            MultibodyComponent::Body(body) => body.get_component_id(),
            MultibodyComponent::Joint(joint) => joint.get_component_id(),
        }
    }    

    fn get_dummy_id(&self) -> &Uuid {
        match self {
            MultibodyComponent::Base(base) => base.get_dummy_id(),
            MultibodyComponent::Body(body) => body.get_dummy_id(),
            MultibodyComponent::Joint(joint) => joint.get_dummy_id(),
        }
    }

    fn get_name_id(&self) -> Uuid {
        match self {
            MultibodyComponent::Base(base) => base.get_name_id(),
            MultibodyComponent::Body(body) => body.get_name_id(),
            MultibodyComponent::Joint(joint) => joint.get_name_id(),
        }
    }

    fn get_node_id(&self) -> &Uuid {
        match self {
            MultibodyComponent::Base(base) => base.get_node_id(),
            MultibodyComponent::Body(body) => body.get_node_id(),
            MultibodyComponent::Joint(joint) => joint.get_node_id(),
        }
    }

    fn inherit_from(&mut self, dummy: &DummyComponent) {
        match self {
            MultibodyComponent::Base(base) => base.inherit_from(dummy),
            MultibodyComponent::Body(body) => body.inherit_from(dummy),
            MultibodyComponent::Joint(joint) => joint.inherit_from(dummy),
        }
    }

    fn set_component_id(&mut self, id: &Uuid) {
        match self {
            MultibodyComponent::Base(base) => base.set_component_id(id),
            MultibodyComponent::Body(body) => body.set_component_id(id),
            MultibodyComponent::Joint(joint) => joint.set_component_id(id),
        }
    }

    fn set_name_id(&mut self, id: &Uuid) {
        match self {
            MultibodyComponent::Base(base) => base.set_name_id(id),
            MultibodyComponent::Body(body) => body.set_name_id(id),
            MultibodyComponent::Joint(joint) => joint.set_name_id(id),
        }
    }

    fn set_node_id(&mut self, id: &Uuid) {
        match self {
            MultibodyComponent::Base(base) => base.set_node_id(id),
            MultibodyComponent::Body(body) => body.set_node_id(id),
            MultibodyComponent::Joint(joint) => joint.set_node_id(id),
        }
    }
    
}

impl MultibodyComponent {
    pub fn from_dummy(component_id: Uuid, name_id: Uuid, node_id: Uuid, dummy: &DummyComponent) -> Self {
        match dummy {
            DummyComponent::Base(_) => MultibodyComponent::Base(Base::new(component_id,dummy.get_id(),name_id,node_id)),
            DummyComponent::Body(component) => MultibodyComponent::Body(Body::from_dummy(component_id,name_id,node_id,component)),
            DummyComponent::Revolute(component) => MultibodyComponent::Joint(Joint::Revolute(Revolute::from_dummy(component_id,name_id,node_id,component))),
        }
    }
}