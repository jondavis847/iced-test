use crate::multibody::{    
    joints::JointParameters,
    MultibodyMeta, MultibodyTrait,
};
use crate::ui::dummies::{DummyComponent, DummyRevolute, DummyTrait};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub enum RevoluteField {
    Name,
    ConstantForce,
    Dampening,
    Omega,
    SpringConstant,
    Theta,
}

#[derive(Debug, Clone, Copy)]
pub struct RevoluteState {
    pub theta: f64,
    pub omega: f64,
}

impl RevoluteState {
    pub fn new(theta: f64, omega: f64) -> Self {
        Self { theta, omega }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Revolute {    
    pub meta: MultibodyMeta,
    pub parameters: JointParameters,
    pub state: RevoluteState,
}

impl Revolute {
    pub fn from_dummy(
        component_id: Uuid,
        node_id: Uuid,
        name_id: Uuid,
        dummy: &DummyRevolute,
    ) -> Self {
        let meta = MultibodyMeta::new(component_id, dummy.get_id(), name_id, node_id);

        let state = RevoluteState::new(
            dummy.theta.parse().unwrap_or(0.0),
            dummy.omega.parse().unwrap_or(0.0),
        );
        let parameters = JointParameters::new(
            dummy.constant_force.parse().unwrap_or(0.0),
            dummy.dampening.parse().unwrap_or(0.0),
            dummy.spring_constant.parse().unwrap_or(0.0),
        );

        Self {            
            meta: meta,
            parameters: parameters,
            state: state,
        }
    }
}

impl MultibodyTrait for Revolute {
    fn connect_from(&mut self, id:Uuid) {
        self.meta.from_id = Some(id);
    }

    fn connect_to(&mut self, id: Uuid) {
        self.meta.to_id = Some(id);
    }

    fn delete_from(&mut self) {
        self.meta.from_id = None;
    }
    fn delete_to(&mut self) {
        self.meta.to_id = None;
    }

    fn get_component_id(&self) -> Uuid {
        self.meta.component_id
    }

    fn get_dummy_id(&self) -> Uuid {
        self.meta.dummy_id
    }

    fn get_from_id(&self) -> Option<Uuid> {
        self.meta.from_id
    }

    fn get_name_id(&self) -> Uuid {
        self.meta.name_id
    }

    fn get_node_id(&self) -> Uuid {
        self.meta.node_id
    }

    fn get_to_id(&self) -> Option<Uuid> {
        self.meta.to_id
    }

    fn inherit_from(&mut self, dummy: &DummyComponent) {
        match dummy {
            DummyComponent::Revolute(_) => {}
            _ => {} // error! must be dummy base
        }
    }

    fn set_component_id(&mut self, id: Uuid) {
        self.meta.component_id = id;
    }

    fn set_name_id(&mut self, id: Uuid) {
        self.meta.name_id = id;
    }

    fn set_node_id(&mut self, id: Uuid) {
        self.meta.node_id = id;
    }

    fn set_system_id(&mut self, id: usize) {
        self.meta.system_id = Some(id);        
    }
}
