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
            mass: dummy.mass.parse().expect("Failed to parse mass"),
            cmx: dummy.cmx.parse().expect("Failed to parse cmx"),
            cmy: dummy.cmy.parse().expect("Failed to parse cmy"),
            cmz: dummy.cmz.parse().expect("Failed to parse cmz"),
            ixx: dummy.ixx.parse().expect("Failed to parse ixx"),
            iyy: dummy.iyy.parse().expect("Failed to parse iyy"),
            izz: dummy.izz.parse().expect("Failed to parse izz"),
            ixy: dummy.ixy.parse().expect("Failed to parse ixy"),
            ixz: dummy.ixz.parse().expect("Failed to parse ixz"),
            iyz: dummy.iyz.parse().expect("Failed to parse iyz"),
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
