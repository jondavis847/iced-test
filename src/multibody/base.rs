use super::{MultibodyMeta, MultibodyTrait};
use crate::ui::dummies::DummyComponent;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct Base {
    pub meta: MultibodyMeta,
}

impl Base {
    pub fn new(component_id: Uuid, dummy_id: Uuid, node_id: Uuid, name_id: Uuid) -> Self {
        let meta = MultibodyMeta::new(component_id, dummy_id, name_id, node_id);
        Self { meta }
    }
}

impl MultibodyTrait for Base {
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
            DummyComponent::Base(_) => {                
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
