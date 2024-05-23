use crate::multibody::{connection::JointConnection, MultibodyMeta, MultibodyTrait};
use uuid::Uuid;
use crate::ui::dummies::{DummyComponent,DummyRevolute, DummyTrait};

#[derive(Debug, Clone, Copy)]
pub struct Revolute {
    pub meta: MultibodyMeta,    
    connection: JointConnection,
}

impl Revolute {
    pub fn from_dummy(component_id: Uuid, node_id: Uuid, name_id: Uuid, dummy: &DummyRevolute) -> Self {
        let meta = MultibodyMeta::new(component_id, dummy.get_id(), name_id, node_id);
        Self {
            meta: meta,
            connection: JointConnection::default(),
        }
    }
}

impl MultibodyTrait for Revolute {
    fn get_component_id(&self) -> &Uuid {
        &self.meta.component_id
    }    

    fn get_dummy_id(&self) -> &Uuid {
        &self.meta.dummy_id
    }

    fn get_name_id(&self) -> Uuid {
        self.meta.name_id
    }

    fn get_node_id(&self) -> &Uuid {
        &self.meta.node_id
    }

    fn inherit_from(&mut self, dummy: &DummyComponent) {
        match dummy {
            DummyComponent::Revolute(_) => {                
            }
            _ => {} // error! must be dummy base
        }
    }

    fn set_component_id(&mut self, id: &Uuid) {
        self.meta.component_id = *id;
    }

    fn set_name_id(&mut self, id: &Uuid) {
        self.meta.name_id = *id;
    }
    
    fn set_node_id(&mut self, id: &Uuid) {
        self.meta.node_id = *id;
    }
    
}



