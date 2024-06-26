use iced::{mouse::Cursor, Point, Rectangle, Size};
use std::collections::HashMap;
use uuid::Uuid;

use super::edge::{Edge, EdgeConnection};
use super::node::Node;
use crate::multibody::{
    joints::Joint, MultibodyComponent, MultibodyErrors, MultibodySystem, MultibodyTrait,
};
use crate::ui::dummies::{DummyComponent, DummyTrait};
use crate::{MouseButton, MouseButtonReleaseEvents};

pub enum GraphMessage {
    EditComponent(Uuid),
}

pub enum GraphErrors {
    BodyInvalidId(Uuid),
    BodyMissingFrom(Uuid),
    IdNotFound(Uuid),
    NoBase,
    NoBaseConnections,
    JointMissingFrom(Uuid),
    JointMissingTo(Uuid),
    JointNoOuterBody(Uuid),
    Multibody(MultibodyErrors),
}

#[derive(Debug, Clone)]
pub struct GraphNode {
    pub component_id: Uuid,
    pub edges: Vec<Uuid>,
    pub node: Node,
}

impl GraphNode {
    fn new(component_id: Uuid, node: Node) -> Self {
        let edges = Vec::new();
        Self {
            component_id,
            edges,
            node,
        }
    }
}

#[derive(Debug)]
pub struct Graph {
    pub bounds: Rectangle,
    pub components: HashMap<Uuid, MultibodyComponent>,
    current_edge: Option<Uuid>,
    pub edges: HashMap<Uuid, Edge>,
    pub is_clicked: bool,
    last_cursor_position: Option<Point>,
    left_clicked_node: Option<Uuid>,
    pub nodes: HashMap<Uuid, GraphNode>,
    right_clicked_node: Option<Uuid>,
    selected_node: Option<Uuid>,
    //zoom: f32,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            bounds: Rectangle::new(Point::new(130.0, 0.0), Size::new(870.0, 1000.0)),
            components: HashMap::new(),
            current_edge: None,
            edges: HashMap::new(),
            is_clicked: false,
            last_cursor_position: None,
            left_clicked_node: None,
            nodes: HashMap::new(),
            right_clicked_node: None,
            selected_node: None,
            //zoom: 1.0,
        }
    }
}

impl Graph {
    pub fn create_multibody_system(&mut self) -> Result<MultibodySystem, GraphErrors> {
        let mut body_counter: usize = 0;
        let mut joint_counter: usize = 0;

        let mut base = None;
        let mut base_joints = Vec::new();
        let mut joints = Vec::<Joint>::new();
        let mut bodies = Vec::<MultibodyComponent>::new(); //Multibody component so it can be base and body

        // verify at least 1 base
        for (_, component) in &self.components {
            match component {
                MultibodyComponent::Base(_) => base = Some(component),
                _ => {}
            }
        }

        let base = match base {
            Some(base) => base,
            None => return Err(GraphErrors::NoBase),
        };

        let base_id = base.get_component_id();

        for (id, component) in &self.components {
            match component {
                MultibodyComponent::Joint(joint) => {
                    // ensure all joints have a from id
                    let from_id = match joint.get_from_id() {
                        Some(id) => id,
                        None => return Err(GraphErrors::JointMissingFrom(*id)),
                    };

                    // ensure all joints have a to id
                    if joint.get_to_id().is_empty() {
                        return Err(GraphErrors::JointMissingTo(*id));
                    };

                    // if the from id equals the base id, the joint is connected to the base
                    if from_id == base_id {
                        base_joints.push(*id);
                    }
                }
                MultibodyComponent::Body(body) => {
                    // ensure all bodies have a from id
                    match body.get_from_id() {
                        Some(_) => {}
                        None => return Err(GraphErrors::BodyMissingFrom(*id)),
                    };
                }
                _ => {}
            }
        }

        // verify something is connected to the base
        if base_joints.is_empty() {
            return Err(GraphErrors::NoBaseConnections);
        }

        // recursively clone, identify, and push the componentns to make the multibody tree
        let result = self.traverse_component(
            base,
            &mut bodies,
            &mut joints,
            &mut body_counter,
            &mut joint_counter,
        );

        match result {
            Ok(_)  => {
                let system = MultibodySystem::new(bodies, joints);
                return Ok(system)
            }
            Err(error) => return Err(error),
        }
    }

    pub fn cursor_moved(&mut self, cursor: Cursor) -> bool {
        let mut redraw = false;
        let cursor_position = cursor.position_in(self.bounds);

        // Handle left-clicked node dragging
        if let Some(clicked_node_id) = self.left_clicked_node {
            if let Some(graphnode) = self.nodes.get_mut(&clicked_node_id) {
                if let Some(position) = cursor_position {
                    graphnode.node.translate_to(position);
                    redraw = true;
                }
            }
        } else if self.is_clicked {
            // Handle graph translating
            if let Some(graph_cursor_position) = cursor_position {
                if let Some(last_position) = self.last_cursor_position {
                    let delta = graph_cursor_position - last_position;
                    self.nodes.iter_mut().for_each(|(_, graphnode)| {
                        graphnode.node.translate_by(delta);
                    });
                    redraw = true;
                }
            }
        }

        // Update last cursor position
        if let Some(position) = cursor_position {
            self.last_cursor_position = Some(position);

            // Handle right-clicked node for edge drawing
            if let Some(clicked_node_id) = self.right_clicked_node {
                if let Some(edge_id) = self.current_edge {
                    if let Some(edge) = self.edges.get_mut(&edge_id) {
                        edge.to = EdgeConnection::Point(position);
                    }
                } else {
                    let new_edge = Edge::new(
                        EdgeConnection::Node(clicked_node_id),
                        EdgeConnection::Point(position),
                    );
                    let new_edge_id = Uuid::new_v4();
                    self.edges.insert(new_edge_id, new_edge);
                    self.current_edge = Some(new_edge_id);

                    // Add the edge to the from node
                    if let Some(from_node) = self.nodes.get_mut(&clicked_node_id) {
                        from_node.edges.push(new_edge_id);
                    }
                }
                redraw = true;
            }
        }

        redraw
    }

    pub fn delete_pressed(&mut self) {
        if let Some(selected_node_id) = self.selected_node.take() {
            if let Some(selected_node) = self.nodes.remove(&selected_node_id) {
                // Remove edges from all nodes and edges collection
                for edge_id in selected_node.edges {
                    for node in self.nodes.values_mut() {
                        node.edges.retain(|x| x != &edge_id);
                    }
                    self.edges.remove(&edge_id);
                }

                if let Some(component) = self.components.get(&selected_node.component_id) {
                    // Remove the component from components
                    self.components.remove(&selected_node.component_id);
                }
            }
        }
    }

    pub fn edit_component(&mut self, dummy: &DummyComponent, component_id: Uuid) -> Result<(), GraphErrors> {
        let component = match self.components.get(&component_id) {
            Some(component) => component,
            None => return Err(GraphErrors::IdNotFound(component_id)),
        };

        if let Some(component) = self.components.get_mut(&component_id) {
            component.inherit_from(dummy);
        }
        Ok(())
    }

    /// Finds a node within snapping distance of the cursor on the graph, if any.
    ///
    /// # Arguments
    ///
    /// * `cursor` - The current position of the cursor.
    /// * `nodes` - A reference to the hashmap containing all nodes in the graph.
    ///
    /// # Returns
    ///
    /// * `Option<Uuid>` - The UUID of the node under the cursor, if any.
    ///
    /// This function checks if the cursor is within the graph's bounds and, if so,
    /// iterates over the nodes to find the first node is in snapping distance of the cursor position.
    /// If such a node is found, its UUID is returned.
    fn get_snappable_node(&self, cursor: Cursor) -> Option<Uuid> {
        // Check if the cursor is within the graph's bounds
        if let Some(cursor_position) = cursor.position_in(self.bounds) {
            // Find the first node that the cursor is in snapping distance of
            return self
                .nodes
                .iter()
                .find(|(_, graphnode)| {
                    let node = &graphnode.node;
                    // Check if the cursor's x and y positions are within the node's bounds
                    cursor_position.x > node.bounds.x
                        && cursor_position.x < node.bounds.x + node.bounds.width
                        && cursor_position.y > node.bounds.y
                        && cursor_position.y < node.bounds.y + node.bounds.height
                })
                // If a node is found, return its UUID
                .map(|(id, _)| *id);
        }
        // If no node is found, return None
        None
    }

    fn is_valid_connection(&self, from_component_id: &Uuid, to_component_id: &Uuid) -> bool {
        if let Some(from_component) = self.components.get(from_component_id) {
            if let Some(to_component) = self.components.get(to_component_id) {
                matches!(
                    (from_component, to_component),
                    (
                        MultibodyComponent::Base(_),
                        MultibodyComponent::Joint(Joint::Revolute(_))
                    ) | (
                        MultibodyComponent::Body(_),
                        MultibodyComponent::Joint(Joint::Revolute(_))
                    ) | (
                        MultibodyComponent::Joint(Joint::Revolute(_)),
                        MultibodyComponent::Base(_)
                    ) | (
                        MultibodyComponent::Joint(Joint::Revolute(_)),
                        MultibodyComponent::Body(_)
                    )
                )
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn left_button_pressed(&mut self, cursor: Cursor) {
        self.left_clicked_node = None;

        if let Some(cursor_position) = cursor.position_in(self.bounds) {
            self.is_clicked = true;
            self.last_cursor_position = Some(cursor_position);

            // Clear the nodes' selected flags and determine the clicked node
            for (id, graphnode) in &mut self.nodes {
                let node = &mut graphnode.node;
                node.is_selected = false;
                node.is_clicked(cursor_position, &MouseButton::Left);

                if node.is_left_clicked {
                    node.is_selected = true;
                    self.left_clicked_node = Some(*id);
                }
            }
        }

        // Update selected_node based on whether a node was clicked
        self.selected_node = self.left_clicked_node;
    }

    pub fn left_button_released(
        &mut self,
        release_event: &MouseButtonReleaseEvents,
        cursor: Cursor,
    ) -> Option<GraphMessage> {
        self.is_clicked = false;
        let mut message = None;

        if let Some(cursor_position) = cursor.position_in(self.bounds) {
            self.last_cursor_position = Some(cursor_position);
        }

        //clear the nodes selected flags, to be reapplied on click
        self.nodes.iter_mut().for_each(|(_, graphnode)| {
            graphnode.node.is_selected = false;
        });

        if let Some(clicked_node_id) = self.left_clicked_node {
            if let Some(graphnode) = self.nodes.get_mut(&clicked_node_id) {
                let clicked_node = &mut graphnode.node;
                match release_event {
                    MouseButtonReleaseEvents::DoubleClick => {
                        clicked_node.is_selected = true;
                        message = Some(GraphMessage::EditComponent(graphnode.component_id));
                    }
                    MouseButtonReleaseEvents::SingleClick => {
                        clicked_node.is_selected = true;
                    }
                    MouseButtonReleaseEvents::Held => {
                        clicked_node.is_selected = true;
                    }
                    MouseButtonReleaseEvents::Nothing => {}
                }
            }
        } else {
            // Clear all selections if no node was clicked
            self.nodes
                .values_mut()
                .for_each(|graphnode| graphnode.node.is_selected = false);
            self.selected_node = None;
        }
        self.left_clicked_node = None;
        message
    }

    pub fn right_button_pressed(&mut self, cursor: Cursor) {
        self.right_clicked_node = None;

        if let Some(cursor_position) = cursor.position_in(self.bounds) {
            for (id, graphnode) in &mut self.nodes {
                let node = &mut graphnode.node;
                node.is_clicked(cursor_position, &MouseButton::Right);
                if node.is_right_clicked {
                    self.right_clicked_node = Some(*id);
                }
            }
            self.last_cursor_position = Some(cursor_position);
        }
    }

    /// Handles the release of the right mouse button, finalizing or canceling an edge creation process.
    ///
    /// This function checks if there is an active edge creation process (stored in `self.current_edge`).
    /// If there is, it attempts to finalize the edge by connecting it to a node near the cursor.
    /// If any step in this process fails, it gracefully exits by removing the edge and resetting the state.
    ///
    /// # Arguments
    ///
    /// * `cursor` - The current position of the cursor.
    pub fn right_button_released(&mut self, cursor: Cursor) {
        // Get the current edge ID if it exists, return if it does not
        let edge_id = match self.current_edge {
            Some(id) => id,
            None => return,
        };

        // Helper function to clean up in case anything goes wrong
        let graceful_exit = |graph: &mut Self| {
            graph.edges.remove(&edge_id);
            graph.current_edge = None;
            graph.right_clicked_node = None;
        };

        // Get the edge if it exists, return if it does not
        let edge = match self.edges.get(&edge_id) {
            Some(edge) => edge,
            None => {
                graceful_exit(self);
                return;
            }
        };

        // Get the from_node_id, return if it is an EdgeConnection::Point
        let from_node_id = match edge.from {
            EdgeConnection::Node(id) => id,
            _ => {
                graceful_exit(self);
                return;
            }
        };

        // Get the component ID of the from node, return if it does not exist
        let from_component_id = match self.nodes.get(&from_node_id).map(|node| node.component_id) {
            Some(id) => id,
            None => {
                graceful_exit(self);
                return;
            }
        };

        // Attempt to get the snappable node near the cursor, return if it does not exist
        let to_node_id = match self.get_snappable_node(cursor) {
            Some(id) => id,
            None => {
                graceful_exit(self);
                return;
            }
        };

        // Get the component ID of the to node, return if it does not exist
        let to_component_id = match self.nodes.get(&to_node_id).map(|node| node.component_id) {
            Some(id) => id,
            None => {
                graceful_exit(self);
                return;
            }
        };

        // Check if the connection is valid between the from and to components
        let valid_connection = self.is_valid_connection(&from_component_id, &to_component_id);

        // Connect the components if the connection is valid
        if valid_connection {
            // Get the from component, return if it does not exist
            let from_component = match self.components.get_mut(&from_component_id) {
                Some(component) => component,
                None => {
                    graceful_exit(self);
                    return;
                }
            };
            from_component.connect_to(to_component_id);

            // Get the to component, return if it does not exist
            let to_component = match self.components.get_mut(&to_component_id) {
                Some(component) => component,
                None => {
                    graceful_exit(self);
                    return;
                }
            };
            to_component.connect_from(from_component_id);

            // Get the to node, return if it does not exist
            let to_node = match self.nodes.get_mut(&to_node_id) {
                Some(node) => node,
                None => {
                    graceful_exit(self);
                    return;
                }
            };
            to_node.edges.push(edge_id);

            // Update the edge to connect to the to_node, return if the edge does not exist
            let edge = match self.edges.get_mut(&edge_id) {
                Some(edge) => edge,
                None => {
                    graceful_exit(self);
                    return;
                }
            };
            edge.to = EdgeConnection::Node(to_node_id);
        }

        // Clear the current edge and right-clicked node state
        self.current_edge = None;
        self.right_clicked_node = None;
    }

    pub fn save_component(&mut self, dummy: &DummyComponent) -> Result<(), GraphErrors> {
        // only do this if we can save the node
        if let Some(last_cursor_position) = self.last_cursor_position {
            // Generate unique IDs for component, node, and name
            let component_id = Uuid::new_v4();
            let node_id = Uuid::new_v4();
            let name_id = Uuid::new_v4();

            // Create the new component from it's dummy
            let new_component = match MultibodyComponent::from_dummy(component_id, dummy, node_id) {
                Ok(component) => component,
                Err(error) => return Err(GraphErrors::Multibody(error)),
            };

            // Calculate the bounds for the new node
            let size = Size::new(100.0, 50.0); // TODO: make width dynamic based on name length
            let top_left = Point::new(
                last_cursor_position.x - size.width / 2.0,
                last_cursor_position.y - size.height / 2.0,
            );
            let bounds = Rectangle::new(top_left, size);

            // Create the new node
            let name = dummy.get_name().to_string();
            let new_node = Node::new(bounds);
            let graph_node = GraphNode::new(component_id, new_node);

            // Insert the new component and node into their respective collections
            self.components.insert(component_id, new_component);
            self.nodes.insert(node_id, graph_node);
        }
        Ok(())        
    }

    // Recursive function to traverse the components
    fn traverse_component(
        &self,
        component: &MultibodyComponent,
        bodies: &mut Vec<MultibodyComponent>,
        joints: &mut Vec<Joint>,
        body_counter: &mut usize,
        joint_counter: &mut usize,
    ) -> Result<(), GraphErrors> {
        let mut result = Ok(());
        match component {
            MultibodyComponent::Body(body) => {
                println!("Traversing Body: {:?}", body);
                let mut body_clone = component.clone();
                body_clone.set_system_id(*body_counter);
                bodies.push(body_clone);
                *body_counter += 1;
                // Traverse the joints connected to this body
                for joint_id in body.get_to_id() {
                    let joint = match self.components.get(joint_id) {
                        Some(joint) => joint,
                        None => return Err(GraphErrors::IdNotFound(*joint_id)),
                    };
                    result =
                        self.traverse_component(joint, bodies, joints, body_counter, joint_counter);
                }
            }

            MultibodyComponent::Base(base) => {
                println!("Traversing Base: {:?}", base);
                let mut base_clone = component.clone();
                base_clone.set_system_id(*body_counter);
                bodies.push(base_clone);
                *body_counter += 1;
                // Traverse the joints connected to this body
                for joint_id in base.get_to_id() {
                    let joint = match self.components.get(joint_id) {
                        Some(joint) => joint,
                        None => return Err(GraphErrors::IdNotFound(*joint_id)),
                    };
                    result =
                        self.traverse_component(joint, bodies, joints, body_counter, joint_counter);
                }
            }
            MultibodyComponent::Joint(joint) => {
                println!("Traversing Joint: {:?}", joint);
                let mut joint_clone = joint.clone();
                joint_clone.set_system_id(*joint_counter);
                *joint_counter += 1;
                joints.push(joint_clone);
                for outer_body_id in joint.get_to_id() {
                    let body = match self.components.get(outer_body_id) {
                        Some(body) => body,
                        None => return Err(GraphErrors::IdNotFound(*outer_body_id)),
                    };
                    result =
                        self.traverse_component(body, bodies, joints, body_counter, joint_counter);
                }
            }
        }
        result
    }

    pub fn window_resized(&mut self, size: Size) {
        self.bounds.height = size.height;
        self.bounds.width = size.width;
    }
}
