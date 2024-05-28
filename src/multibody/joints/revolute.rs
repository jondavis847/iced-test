use crate::multibody::{
    connection::JointConnection,
    joints::{JointParameters, JointTrait},
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
    connection: JointConnection,
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
            connection: JointConnection::default(),
            meta: meta,
            parameters: parameters,
            state: state,
        }
    }
}

impl JointTrait for Revolute {
    fn connect_from(&mut self, from_id: Uuid) {
        self.connection.inner_body = Some(from_id);
    }
    fn connect_to(&mut self, to_id: Uuid) {
        self.connection.outer_body = Some(to_id);
    }

    fn delete_from(&mut self) {
        self.connection.inner_body = None;
    }
    fn delete_to(&mut self) {
        self.connection.outer_body = None;
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
            DummyComponent::Revolute(_) => {}
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
