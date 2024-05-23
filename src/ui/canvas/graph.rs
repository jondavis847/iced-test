use iced::{mouse::Cursor, Point, Rectangle, Size};
use std::collections::HashMap;
use uuid::Uuid;

use super::edge::{Edge, EdgeConnection};
use super::node::Node;
use crate::multibody::{joints::Joint, MultibodyComponent, MultibodyTrait};
use crate::ui::dummies::{DummyComponent, DummyTrait};
use crate::{MouseButton, MouseButtonReleaseEvents};

pub enum GraphMessage {
    EditComponent(Uuid),
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
    pub names: HashMap<Uuid, String>,
    pub nodes: HashMap<Uuid, GraphNode>,
    right_clicked_node: Option<Uuid>,
    selected_node: Option<Uuid>,
    zoom: f32,
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
            names: HashMap::new(),
            nodes: HashMap::new(),
            right_clicked_node: None,
            selected_node: None,
            zoom: 1.0,
        }
    }
}

impl Graph {
    pub fn cursor_moved(&mut self, cursor: Cursor) -> bool {
        let mut redraw = false;
        if let Some(clicked_node_id) = self.left_clicked_node {
            // a node is clicked and being dragged
            if let Some(graphnode) = self.nodes.get_mut(&clicked_node_id) {
                let clicked_node = &mut graphnode.node;
                if let Some(cursor_position) = cursor.position_in(self.bounds) {
                    clicked_node.translate_to(cursor_position);
                    redraw = true;
                }
            }
        } else {
            // no node is clicked, graph is translating if the cursor is clicked on the graph
            if self.is_clicked {
                if let Some(graph_cursor_position) = cursor.position_in(self.bounds) {
                    if let Some(last_cursor_position) = self.last_cursor_position {
                        let delta = graph_cursor_position - last_cursor_position;
                        self.nodes.iter_mut().for_each(|(_, graphnode)| {
                            graphnode.node.translate_by(delta);
                        });
                        redraw = true;
                    }
                }
            }
        }
        if let Some(cursor_position) = cursor.position_in(self.bounds) {
            self.last_cursor_position = Some(cursor_position);
            if let Some(clicked_node_id) = self.right_clicked_node {
                //edge is being drawn
                if let Some(edge_id) = self.current_edge {
                    //keep moving current_edge
                    if let Some(edge) = self.edges.get_mut(&edge_id) {
                        edge.to = EdgeConnection::Point(cursor_position);
                    }
                } else {
                    //create a new edge
                    let new_edge = Edge::new(
                        EdgeConnection::Node(clicked_node_id),
                        EdgeConnection::Point(cursor_position),
                    );
                    let new_edge_id = Uuid::new_v4();
                    self.edges.insert(new_edge_id, new_edge);
                    self.current_edge = Some(new_edge_id);
                    //add the edge to the from node so that if from node is deleted, edge is deleted
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

                // Remove the name from names
                if let Some(component) = self.components.get(&selected_node.component_id) {
                    let name_id = component.get_name_id();
                    self.names.remove(&name_id);

                    // Remove the component from components
                    self.components.remove(&selected_node.component_id);
                }
            }
        }
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
                        message = Some(GraphMessage::EditComponent(clicked_node_id));
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

    pub fn right_button_released(&mut self, cursor: Cursor) {
        if let Some(edge_id) = self.current_edge {
            if let Some(snappable_node_id) = self.get_snappable_node(cursor) {
                let to_node_modal = self
                    .nodes
                    .get(&snappable_node_id)
                    .map(|node| node.component_id);

                if let Some(to_node_modal) = to_node_modal {
                    let valid_connection = {
                        if let Some(active_edge) = self.edges.get(&edge_id) {
                            match active_edge.from {
                                EdgeConnection::Node(node_id) => {
                                    self.nodes.get(&node_id).map_or(false, |from_node| {
                                        self.is_valid_connection(
                                            &from_node.component_id,
                                            &to_node_modal,
                                        )
                                    })
                                }
                                EdgeConnection::Point(_) => false,
                            }
                        } else {
                            false
                        }
                    };

                    if valid_connection {
                        if let Some(active_edge) = self.edges.get_mut(&edge_id) {
                            if let EdgeConnection::Node(_) = active_edge.from {
                                if let Some(to_node) = self.nodes.get_mut(&snappable_node_id) {
                                    to_node.edges.push(edge_id);
                                    active_edge.to = EdgeConnection::Node(snappable_node_id);
                                }
                            }
                        }
                    } else {
                        self.edges.remove(&edge_id);
                    }
                }
            } else {
                self.edges.remove(&edge_id);
            }
        }

        self.current_edge = None;
        self.right_clicked_node = None;
    }

    pub fn save_component(&mut self, dummy: &DummyComponent) {
        // only do this if we can save the node
        if let Some(last_cursor_position) = self.last_cursor_position {
            // Generate unique IDs for component, node, and name
            let component_id = Uuid::new_v4();
            let node_id = Uuid::new_v4();
            let name_id = Uuid::new_v4();

            // Create the new component from it's dummy
            let new_component =
                MultibodyComponent::from_dummy(component_id, node_id, name_id, dummy);

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
            self.names.insert(name_id, name);
            self.components.insert(component_id, new_component);
            self.nodes.insert(node_id, graph_node);
        }
    }

    pub fn window_resized(&mut self, size: Size) {
        self.bounds.height = size.height;
        self.bounds.width = size.width;
    }
}
