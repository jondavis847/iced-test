use crate::ui::dummies::{DummyBody, DummyTrait};

use super::{MultibodyMeta, MultibodyTrait};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub struct Body {
    pub meta: MultibodyMeta,
    pub mass: f64,
    pub cmx: f64,
    pub cmy: f64,
    pub cmz: f64,
    pub ixx: f64,
    pub iyy: f64,
    pub izz: f64,
    pub ixy: f64,
    pub ixz: f64,
    pub iyz: f64,
}

impl Body {
    pub fn from_dummy(component_id: Uuid, node_id: Uuid, name_id: Uuid, dummy: &DummyBody) -> Self {
        let meta = MultibodyMeta::new(component_id, dummy.get_id(),name_id, node_id);
        Self {
            meta,
            mass: dummy.mass.parse().unwrap_or(1.0),
            cmx: dummy.cmx.parse().unwrap_or(0.0),
            cmy: dummy.cmy.parse().unwrap_or(0.0),
            cmz: dummy.cmz.parse().unwrap_or(0.0),
            ixx: dummy.ixx.parse().unwrap_or(1.0),
            iyy: dummy.iyy.parse().unwrap_or(1.0),
            izz: dummy.izz.parse().unwrap_or(1.0),
            ixy: dummy.ixy.parse().unwrap_or(0.0),
            ixz: dummy.ixz.parse().unwrap_or(0.0),
            iyz: dummy.iyz.parse().unwrap_or(0.0),
        }
    }
}

impl MultibodyTrait for Body {
    fn get_component_id(&self) -> &Uuid {
        &self.meta.component_id
    }
    fn set_component_id(&mut self, id: &Uuid) {
        self.meta.component_id = *id;
    }

    fn get_dummy_id(&self) -> &Uuid {
        &self.meta.dummy_id
    }

    fn get_node_id(&self) -> &Uuid {
        &self.meta.node_id
    }
    fn set_node_id(&mut self, id: &Uuid) {
        self.meta.node_id = *id;
    }

    fn get_name_id(&self) -> &Uuid {
        &self.meta.name_id
    }
    fn set_name_id(&mut self, id: &Uuid) {
        self.meta.name_id = *id;
    }
}
