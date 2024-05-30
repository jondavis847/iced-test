use uuid::Uuid;

pub mod base;
pub mod body;
pub mod joints;

use base::Base;
use body::Body;
use joints::{Joint, revolute::Revolute};

use crate::ui::dummies::{DummyComponent, DummyTrait};
#[derive(Debug, Clone, Copy)]
pub struct MultibodyMeta {
    component_id: Uuid,        
    dummy_id: Uuid,
    from_id: Option<Uuid>,
    name_id: Uuid,        
    node_id: Uuid,    
    system_id: Option<usize>,
    to_id: Option<Uuid>,
}

impl MultibodyMeta {
    pub fn new(component_id: Uuid, dummy_id: Uuid, name_id: Uuid, node_id: Uuid) -> Self {
        let from_id = None;
        let system_id = None;
        let to_id = None;
        Self {
            component_id,                        
            dummy_id,
            from_id,
            name_id,
            node_id,
            system_id,
            to_id,
        }
    }
}

pub trait MultibodyTrait {
    fn connect_from(&mut self, id: Uuid);
    fn connect_to(&mut self, id: Uuid);
    fn delete_from(&mut self);
    fn delete_to(&mut self);
    fn get_component_id(&self) -> Uuid;
    fn get_dummy_id(&self) -> Uuid;    
    fn get_from_id(&self) -> Option<Uuid>;    
    fn get_name_id(&self) -> Uuid;
    fn get_node_id(&self) -> Uuid;    
    fn get_to_id(&self) -> Option<Uuid>;
    fn inherit_from(&mut self, dummy: &DummyComponent);
    fn set_component_id(&mut self, id: Uuid);    
    fn set_name_id(&mut self, id: Uuid);    
    fn set_node_id(&mut self, id: Uuid);
    fn set_system_id(&mut self, id: usize);
}

#[derive(Debug, Clone, Copy)]
pub enum MultibodyComponent {
    Base(Base),
    Body(Body),
    Joint(Joint),
}

impl MultibodyTrait for MultibodyComponent {
    fn connect_from(&mut self, from_id: Uuid) {
        match self {
            MultibodyComponent::Base(base) => base.connect_from(from_id),
            MultibodyComponent::Body(body) => body.connect_from(from_id),
            MultibodyComponent::Joint(joint) => joint.connect_from(from_id),
        }
    }
    fn connect_to(&mut self, to_id: Uuid) {
        match self {
            MultibodyComponent::Base(base) => base.connect_to(to_id),
            MultibodyComponent::Body(body) => body.connect_to(to_id),
            MultibodyComponent::Joint(joint) => joint.connect_to(to_id),
        }
    }

    fn delete_from(&mut self) {
        match self {
            MultibodyComponent::Base(base) => base.delete_from(),
            MultibodyComponent::Body(body) => body.delete_from(),
            MultibodyComponent::Joint(joint) => joint.delete_from(),
        }
    }

    fn delete_to(&mut self) {
        match self {
            MultibodyComponent::Base(base) => base.delete_to(),
            MultibodyComponent::Body(body) => body.delete_to(),
            MultibodyComponent::Joint(joint) => joint.delete_to(),
        }
    }
    fn get_component_id(&self) -> Uuid {
        match self {
            MultibodyComponent::Base(base) => base.get_component_id(),
            MultibodyComponent::Body(body) => body.get_component_id(),
            MultibodyComponent::Joint(joint) => joint.get_component_id(),
        }
    }    

    fn get_dummy_id(&self) -> Uuid {
        match self {
            MultibodyComponent::Base(base) => base.get_dummy_id(),
            MultibodyComponent::Body(body) => body.get_dummy_id(),
            MultibodyComponent::Joint(joint) => joint.get_dummy_id(),
        }
    }

    fn get_from_id(&self) -> Option<Uuid> {
        match self {
            MultibodyComponent::Base(base) => base.get_from_id(),
            MultibodyComponent::Body(body) => body.get_from_id(),
            MultibodyComponent::Joint(joint) => joint.get_from_id(),
        }
    }

    fn get_name_id(&self) -> Uuid {
        match self {
            MultibodyComponent::Base(base) => base.get_name_id(),
            MultibodyComponent::Body(body) => body.get_name_id(),
            MultibodyComponent::Joint(joint) => joint.get_name_id(),
        }
    }

    fn get_node_id(&self) -> Uuid {
        match self {
            MultibodyComponent::Base(base) => base.get_node_id(),
            MultibodyComponent::Body(body) => body.get_node_id(),
            MultibodyComponent::Joint(joint) => joint.get_node_id(),
        }
    }

    fn get_to_id(&self) -> Option<Uuid> {
        match self {
            MultibodyComponent::Base(base) => base.get_to_id(),
            MultibodyComponent::Body(body) => body.get_to_id(),
            MultibodyComponent::Joint(joint) => joint.get_to_id(),
        }
    }

    fn inherit_from(&mut self, dummy: &DummyComponent) {
        match self {
            MultibodyComponent::Base(base) => base.inherit_from(dummy),
            MultibodyComponent::Body(body) => body.inherit_from(dummy),
            MultibodyComponent::Joint(joint) => joint.inherit_from(dummy),
        }
    }

    fn set_component_id(&mut self, id: Uuid) {
        match self {
            MultibodyComponent::Base(base) => base.set_component_id(id),
            MultibodyComponent::Body(body) => body.set_component_id(id),
            MultibodyComponent::Joint(joint) => joint.set_component_id(id),
        }
    }

    fn set_name_id(&mut self, id: Uuid) {
        match self {
            MultibodyComponent::Base(base) => base.set_name_id(id),
            MultibodyComponent::Body(body) => body.set_name_id(id),
            MultibodyComponent::Joint(joint) => joint.set_name_id(id),
        }
    }

    fn set_node_id(&mut self, id: Uuid) {
        match self {
            MultibodyComponent::Base(base) => base.set_node_id(id),
            MultibodyComponent::Body(body) => body.set_node_id(id),
            MultibodyComponent::Joint(joint) => joint.set_node_id(id),
        }
    }

    fn set_system_id(&mut self, id: usize) {
        match self {
            MultibodyComponent::Base(base) => base.set_system_id(id),
            MultibodyComponent::Body(body) => body.set_system_id(id),
            MultibodyComponent::Joint(joint) => joint.set_system_id(id),
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

pub struct MultibodySystem {
    base: Base,
    bodies: Vec<Body>,
    joints: Vec<Joint>,
}