use crate::multibody::{joints::Joint, MultibodyComponent, MultibodyTrait};
use crate::ui::canvas::graph::Graph;
use uuid::Uuid;

/// DummyComponents are like MultibodyComponents but with String fields
/// for editing in the text inputs rather than numeric values
#[derive(Debug, Clone)]
pub enum DummyComponent {
    Base(DummyBase),
    Body(DummyBody),
    Revolute(DummyRevolute),
}

pub trait DummyTrait {
    fn clear(&mut self);
    fn get_id(&self) -> Uuid;
    fn get_name(&self) -> &str;
    fn inherit_from(&mut self, component_id: &Uuid, graph: &Graph); // these args suck, but has to be this way sicne names is stored separately in graph for performance
    fn set_name(&mut self, name: &str);
}

impl DummyTrait for DummyComponent {
    fn clear(&mut self) {
        match self {            
            DummyComponent::Base(component) => component.clear(),
            DummyComponent::Body(component) => component.clear(),
            DummyComponent::Revolute(component) => component.clear(),
        }
    }

    fn get_id(&self) -> Uuid {
        match self {            
            DummyComponent::Base(component) => component.get_id(),
            DummyComponent::Body(component) => component.get_id(),
            DummyComponent::Revolute(component) => component.get_id(),
        }
    }

    fn get_name(&self) -> &str {
        match self {
            DummyComponent::Base(component) => component.get_name(),
            DummyComponent::Body(component) => component.get_name(),
            DummyComponent::Revolute(component) => component.get_name(),
        }
    }

    fn inherit_from(&mut self, component_id: &Uuid, graph: &Graph) {
        match self {
            DummyComponent::Base(dummy) => dummy.inherit_from(component_id, graph),
            DummyComponent::Body(dummy) => dummy.inherit_from(component_id, graph),
            DummyComponent::Revolute(dummy) => dummy.inherit_from(component_id, graph),
        }
    }

    fn set_name(&mut self, name: &str) {
        match self {
            DummyComponent::Base(component) => component.set_name(name),
            DummyComponent::Body(component) => component.set_name(name),
            DummyComponent::Revolute(component) => component.set_name(name),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct DummyBase {
    id: Uuid,
    name: String,
}

impl DummyBase {
    pub fn new(id: Uuid) -> Self {
        Self {
            id: id,
            ..Default::default()
        }
    }
}

impl DummyTrait for DummyBase {
    fn clear(&mut self) {
        self.name = String::new();
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn inherit_from(&mut self, component_id: &Uuid, graph: &Graph) {
        if let Some(component) = graph.components.get(component_id) {
            match component {
                MultibodyComponent::Base(_) => {
                    if let Some(name) = graph.names.get(&component.get_name_id()) {
                        self.set_name(name);
                    }

                }
                _ => {} // TODO: error! must be a base
            }
        }
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

#[derive(Default, Debug, Clone)]
pub struct DummyBody {
    id: Uuid,
    pub name: String,
    pub mass: String,
    pub cmx: String,
    pub cmy: String,
    pub cmz: String,
    pub ixx: String,
    pub iyy: String,
    pub izz: String,
    pub ixy: String,
    pub ixz: String,
    pub iyz: String,
}

impl DummyBody {
    pub fn new(id: Uuid) -> Self {
        Self {
            id: id,
            ..Default::default()
        }
    }
}

impl DummyTrait for DummyBody {
    fn clear(&mut self) {
        self.name = String::new();
        self.mass = String::new();
        self.cmx = String::new();
        self.cmy = String::new();
        self.cmz = String::new();
        self.ixx = String::new();
        self.iyy = String::new();
        self.izz = String::new();
        self.ixy = String::new();
        self.ixz = String::new();
        self.iyz = String::new();
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn inherit_from(&mut self, component_id: &Uuid, graph: &Graph) {
        if let Some(component) = graph.components.get(component_id) {
            match component {
                MultibodyComponent::Body(body) => {
                    if let Some(name) = graph.names.get(&component.get_name_id()) {
                        self.set_name(name);
                    }
                    self.mass = body.mass.to_string();
                    self.cmx = body.cmx.to_string();
                    self.cmy = body.cmy.to_string();
                    self.cmz = body.cmz.to_string();
                    self.ixx = body.ixx.to_string();
                    self.iyy = body.iyy.to_string();
                    self.izz = body.izz.to_string();
                    self.ixy = body.ixy.to_string();
                    self.ixz = body.ixz.to_string();
                    self.iyz = body.iyz.to_string();
                }
                _ => {} // TODO: error! must be a base
            }
        }
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

#[derive(Default, Debug, Clone)]
pub struct DummyRevolute {
    id: Uuid,
    pub name: String,
}

impl DummyRevolute {
    pub fn new(id: Uuid) -> Self {
        Self {
            id: id,
            ..Default::default()
        }
    }
}

impl DummyTrait for DummyRevolute {
    fn clear(&mut self) {
        self.name = String::new();
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn inherit_from(&mut self, component_id: &Uuid, graph: &Graph) {
        if let Some(component) = graph.components.get(component_id) {
            match component {
                MultibodyComponent::Joint(joint) => match joint {
                    Joint::Revolute(_) => {
                        if let Some(name) = graph.names.get(&component.get_name_id()) {
                            self.set_name(name);
                        }                        
                    }
                    //_ => {} //TODO: error! must be a revolute
                }
                _ => {} // TODO: error! must be a joint
            }
        }
    }

    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}
