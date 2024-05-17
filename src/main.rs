//#![windows_subsystem = "windows"]
//#![warn(missing_docs)]

use iced::{
    alignment, font, keyboard,
    mouse::Cursor,
    widget::{
        button,
        canvas::{Cache, Canvas},
        container, text, text_input, Column, Row,
    },
    window, Application, Command, Element, Length, Point, Rectangle, Settings, Size, Subscription,
};

use iced_aw::{card, modal};
use multibody::Revolute;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use uuid::Uuid;

mod multibody;
mod ui;

use crate::multibody::{Base, Body, Joint};
use crate::ui::canvas::edge::{Edge, EdgeConnection};
use crate::ui::canvas::graph::Graph;
use crate::ui::canvas::node::Node;
use crate::ui::canvas::node_bar::{NodeBar, NodeBarMap};
use crate::ui::canvas::GraphCanvas;
use crate::ui::modals::Modals;

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.antialiasing = true;
    IcedTest::run(settings)
}

// Define the possible user interactions
#[derive(Debug, Clone)]
enum Message {
    BodyNameInputChanged(String),
    BodyMassInputChanged(String),
    BodyCmxInputChanged(String),
    BodyCmyInputChanged(String),
    BodyCmzInputChanged(String),
    BodyIxxInputChanged(String),
    BodyIyyInputChanged(String),
    BodyIzzInputChanged(String),
    BodyIxyInputChanged(String),
    BodyIxzInputChanged(String),
    BodyIyzInputChanged(String),
    RevoluteNameInputChanged(String),
    LeftButtonPressed(Cursor),
    LeftButtonReleased(Cursor),
    RightButtonPressed(Cursor),
    RightButtonReleased(Cursor),
    CursorMoved(Cursor),
    CloseModal,
    DeletePressed,
    EnterPressed,
    FontLoaded(Result<(), font::Error>),
    Loaded(Result<(), String>),
    SaveBase,
    SaveBody(Body),
    SaveRevolute(Revolute),
    WindowResized(Size),
}

#[derive(Debug)]
enum IcedTest {
    Loading,
    Loaded(AppState),
}

#[derive(Debug)]
struct AppState {
    base: Option<Base>,
    bodies: HashMap<Uuid, crate::multibody::Body>,
    cache: Cache,
    //connections: HashMap<Uuid,Connection>,
    counter_body: usize,
    counter_revolute: usize,
    joints: HashMap<Uuid, crate::multibody::Joint>,
    left_clicked_node: Option<Uuid>,
    left_clicked_time_1: Option<Instant>,
    left_clicked_time_2: Option<Instant>,
    right_clicked_node: Option<Uuid>,
    middle_clicked_node: Option<Uuid>,
    selected_node: Option<Uuid>,
    current_edge: Option<Uuid>,
    edges: HashMap<Uuid, Edge>,
    graph: Graph,
    is_pressed: bool,
    last_graph_cursor_position: Point,
    modal: Option<Uuid>, //uuid of node, modal is owned by node
    nodebar: NodeBar,
    nodes: HashMap<Uuid, Node>,
    theme: crate::ui::theme::Theme,
}

impl Default for AppState {
    fn default() -> Self {
        let nodebar = NodeBar::default();
        let default_nodes = default_nodes(nodebar.clone().map);
        Self {
            base: None,
            bodies: HashMap::new(),
            joints: HashMap::new(),
            cache: Cache::new(),
            counter_body: 0,
            counter_revolute: 0,
            left_clicked_node: None,
            left_clicked_time_1: None,
            left_clicked_time_2: None,
            right_clicked_node: None,
            middle_clicked_node: None,
            selected_node: None,
            current_edge: None,
            edges: HashMap::new(),
            graph: Graph::default(),
            is_pressed: false,
            last_graph_cursor_position: Point::default(),
            modal: None,
            nodebar: nodebar,
            nodes: default_nodes,
            theme: crate::ui::theme::Theme::ORANGE,
            //theme: crate::ui::canvas::themes::Themes::cyberpunk(),
        }
    }
}

enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug)]
enum MouseButtonReleaseEvents {
    SingleClick,
    DoubleClick,
    Held,
}

impl AppState {
    pub fn get_clicked_node(&mut self, cursor: Cursor, mouse_button: &MouseButton) {
        match mouse_button {
            MouseButton::Left => self.left_clicked_node = None,
            MouseButton::Right => self.right_clicked_node = None,
            MouseButton::Middle => self.middle_clicked_node = None,
        }

        self.nodes.iter_mut().for_each(|(key, node)| {
            if node.is_nodebar {
                // use canvas position
                if let Some(cursor_position) = cursor.position() {
                    node.is_clicked(cursor_position, mouse_button);
                    match mouse_button {
                        MouseButton::Left => {
                            if node.is_left_clicked {
                                self.left_clicked_node = Some(key.clone());
                            }
                        }
                        MouseButton::Right => {
                            if node.is_right_clicked {
                                self.right_clicked_node = Some(key.clone());
                            }
                        }
                        MouseButton::Middle => {
                            if node.is_middle_clicked {
                                self.middle_clicked_node = Some(key.clone());
                            }
                        }
                    }
                }
            } else {
                // use graph position
                if let Some(cursor_position) = cursor.position_in(self.graph.bounds) {
                    node.is_clicked(cursor_position, mouse_button);
                    match mouse_button {
                        MouseButton::Left => {
                            if node.is_left_clicked {
                                self.left_clicked_node = Some(key.clone());
                            }
                        }
                        MouseButton::Right => {
                            if node.is_right_clicked {
                                self.right_clicked_node = Some(key.clone());
                            }
                        }
                        MouseButton::Middle => {
                            if node.is_middle_clicked {
                                self.middle_clicked_node = Some(key.clone());
                            }
                        }
                    }
                }
            }
        });
    }

    pub fn cursor_moved(&mut self, cursor: Cursor) {
        if let Some(clicked_node_id) = self.left_clicked_node {
            // a node is clicked and being dragged
            if let Some(clicked_node) = self.nodes.get_mut(&clicked_node_id) {
                if clicked_node.is_nodebar {
                    if let Some(cursor_position) = cursor.position() {
                        clicked_node.translate_to(cursor_position);
                        self.cache.clear();
                    }
                } else {
                    if let Some(cursor_position) = cursor.position_in(self.graph.bounds) {
                        clicked_node.translate_to(cursor_position);
                        self.cache.clear();
                        self.last_graph_cursor_position = cursor_position;
                    }
                }
            }
        } else {
            // no node is clicked, graph is translating if the cursor is clicked on the graph
            if self.is_pressed {
                if let Some(graph_cursor_position) = cursor.position_in(self.graph.bounds) {
                    let delta = graph_cursor_position - self.last_graph_cursor_position;
                    self.nodes.iter_mut().for_each(|(_, node)| {
                        if !node.is_nodebar {
                            node.translate_by(delta);
                        }
                    });
                    self.last_graph_cursor_position = graph_cursor_position;
                    self.cache.clear();
                }
            }
        }
        if let Some(cursor_position) = cursor.position_in(self.graph.bounds) {
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
                self.cache.clear();
            }
        }
    }
    pub fn left_button_pressed(&mut self, cursor: Cursor) {
        self.is_pressed = true;

        if let Some(left_clicked_time_1) = self.left_clicked_time_1 {
            if left_clicked_time_1.elapsed() > Duration::from_millis(200) {
                // too long for double click, just a single click
                self.left_clicked_time_1 = Some(Instant::now());
            } else {
                //within double click, but need to wait for second release
                self.left_clicked_time_2 = Some(Instant::now());
            }
        } else {
            // self.left_clicked_time_1 was None
            self.left_clicked_time_1 = Some(Instant::now());
        }

        self.get_clicked_node(cursor, &MouseButton::Left);
        if let Some(selected_node) = self.left_clicked_node {
            self.selected_node = Some(selected_node);
        } else {
            self.selected_node = None;
        }
        if let Some(graph_cursor_position) = cursor.position_in(self.graph.bounds) {
            self.last_graph_cursor_position = graph_cursor_position;
        }
    }

    pub fn left_button_released(&mut self, cursor: Cursor) {
        self.is_pressed = false;

        // Determine the type of mouse button release event
        let release_event = match (self.left_clicked_time_1, self.left_clicked_time_2) {
            (Some(clicked_time_1), Some(clicked_time_2)) => {
                let clicked_elapsed_time_1 = clicked_time_1.elapsed();
                let clicked_elapsed_time_2 = clicked_time_2.elapsed();

                if clicked_elapsed_time_1 <= Duration::from_millis(500)
                    && clicked_elapsed_time_2 <= Duration::from_millis(200)
                {
                    MouseButtonReleaseEvents::DoubleClick
                } else {
                    MouseButtonReleaseEvents::SingleClick
                }
            }
            (Some(clicked_time_1), None) => {
                if clicked_time_1.elapsed() <= Duration::from_millis(200) {
                    MouseButtonReleaseEvents::SingleClick
                } else {
                    MouseButtonReleaseEvents::Held
                }
            }
            _ => MouseButtonReleaseEvents::Held,
        };

        // Reset click times appropriately
        if matches!(
            release_event,
            MouseButtonReleaseEvents::SingleClick | MouseButtonReleaseEvents::DoubleClick
        ) {
            self.left_clicked_time_1 = None;
            self.left_clicked_time_2 = None;
        }

        // Handle the click event on the node
        if let Some(clicked_node_id) = self.left_clicked_node {
            // First clear any selected_nodes
            self.nodes.iter_mut().for_each(|(_, node)| {
                node.is_selected = false;
            });
            if let Some(clicked_node) = self.nodes.get_mut(&clicked_node_id) {
                match release_event {
                    MouseButtonReleaseEvents::DoubleClick => {
                        // Handle double click if needed
                    }
                    MouseButtonReleaseEvents::SingleClick => {
                        if !clicked_node.is_nodebar {
                            clicked_node.is_selected = true;
                        }
                    }
                    MouseButtonReleaseEvents::Held => {
                        if clicked_node.is_nodebar {
                            if let Some(cursor_position) = cursor.position_in(self.graph.bounds) {
                                self.last_graph_cursor_position = cursor_position;
                            }
                            self.modal = Some(clicked_node_id);
                        }
                        clicked_node.drop();
                    }
                }
            }
            self.left_clicked_node = None;
        } else {
            // Clear all selections if no node was clicked
            self.nodes
                .values_mut()
                .for_each(|node| node.is_selected = false);
            self.selected_node = None;
        }

        // Clear the cache
        self.cache.clear();
    }

    pub fn right_button_pressed(&mut self, cursor: Cursor) {
        self.get_clicked_node(cursor, &MouseButton::Right);
        if let Some(graph_cursor_position) = cursor.position_in(self.graph.bounds) {
            self.last_graph_cursor_position = graph_cursor_position;
        }
    }

    pub fn right_button_released(&mut self, cursor: Cursor) {
        if let Some(edge_id) = self.current_edge {
            if let Some(snappable_node_id) = self.get_snappable_node(cursor, &self.nodes) {
                let to_node_modal = self
                    .nodes
                    .get(&snappable_node_id)
                    .map(|node| node.modal.clone());

                if let Some(to_node_modal) = to_node_modal {
                    let valid_connection = {
                        if let Some(active_edge) = self.edges.get(&edge_id) {
                            match active_edge.from {
                                EdgeConnection::Node(node_id) => {
                                    self.nodes.get(&node_id).map_or(false, |from_node| {
                                        self.is_valid_connection(&from_node.modal, &to_node_modal)
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
        self.cache.clear();
    }

    fn is_valid_connection(&self, from_modal: &Modals, to_modal: &Modals) -> bool {
        matches!(
            (from_modal, to_modal),
            (Modals::Base, Modals::Revolute(_))
                | (Modals::Body(_), Modals::Revolute(_))
                | (Modals::Revolute(_), Modals::Base)
                | (Modals::Revolute(_), Modals::Body(_))
        )
    }

    pub fn delete_pressed(&mut self) {
        if let Some(selected_node_id) = self.selected_node {
            // Collect edges to be removed
            let edges_to_remove = if let Some(selected_node) = self.nodes.get(&selected_node_id) {
                selected_node.edges.clone()
            } else {
                Vec::new()
            };

            // Remove edges from all nodes and edges collection
            for edge_id in edges_to_remove {
                for node in self.nodes.values_mut() {
                    node.edges.retain(|x| x != &edge_id);
                }
                self.edges.remove(&edge_id);
            }

            // Remove the selected node itself
            self.nodes.remove(&selected_node_id);
            self.selected_node = None;

            // Clear the cache
            self.cache.clear();
        }
    }

    /// Handles the enter key press event.
    ///
    /// This function is called when the enter key is pressed. It checks if a modal is active,
    /// retrieves the corresponding node and modal, and then saves the data based on the modal type.
    /// Finally, it clears the cache.
    pub fn enter_pressed(&mut self) {
        if let Some(modal_node_id) = self.modal {
            // If a modal is active, retrieve the corresponding node
            if let Some(modal_node) = self.nodes.get(&modal_node_id) {
                // Clone the modal to avoid borrowing issues
                match modal_node.modal.clone() {
                    Modals::Base => self.save_base(),
                    Modals::Body(modal) => self.save_body(modal),
                    Modals::Revolute(modal) => self.save_revolute(modal),
                }
                // Clear the cache after saving the data
                self.cache.clear();
            }
        }
    }

    /// Saves a new body to the canvas.
    ///
    /// This function is called when adding a new body to the canvas. It ensures that the body has
    /// a unique name if it doesn't already have one, calculates its position, creates a new node
    /// for it, and inserts both the body and the node into their respective collections.
    ///
    /// # Parameters
    ///
    /// * `orig_body` - The original body to be added to the canvas.
    ///
    /// # Behavior
    ///
    /// - If the body's name is empty, it assigns a unique name in the format `bodyN`, where `N` is
    ///   a counter incremented for each new body.
    /// - The function calculates the bounds for the new node based on the last graph cursor position.
    /// - It creates a new node and assigns a unique ID to both the node and the body.
    /// - It inserts the new body and node into the `bodies` and `nodes` collections respectively.
    /// - It clears the modal and the cache after the body is saved.
    pub fn save_body(&mut self, mut body: Body) {
        // Ensure the body has a unique name if it's empty
        if body.name.is_empty() {
            self.counter_body += 1;
            body.name = format!("body{}", self.counter_body);
        }

        // Create the body modal with the updated body
        let body_modal = Modals::Body(body.clone());

        // Calculate the bounds for the new node
        let size = Size::new(100.0, 50.0); // TODO: make width dynamic based on name length
        let top_left = Point::new(
            self.last_graph_cursor_position.x - size.width / 2.0,
            self.last_graph_cursor_position.y - size.height / 2.0,
        );
        let bounds = Rectangle::new(top_left, size);

        // Generate unique IDs for the node and body
        let node_id = Uuid::new_v4();
        let body_id = Uuid::new_v4();

        // Update the body with the new node ID
        body.node = node_id.clone();

        // Create the new node
        let node = Node::new(bounds, None, false, body.name.clone(), body_modal);

        // Insert the new body and node into their respective collections
        self.bodies.insert(body_id, body);
        self.nodes.insert(node_id, node);

        // Clear the modal and cache
        self.modal = None;
        self.cache.clear();
    }

    pub fn save_base(&mut self) {
        let size = Size::new(100.0, 50.0); //TODO: make width dynamic based on name length
        let top_left = Point::new(
            self.last_graph_cursor_position.x - size.width / 2.0,
            self.last_graph_cursor_position.y - size.height / 2.0,
        );
        let bounds = Rectangle::new(top_left, size);

        let node_id = Uuid::new_v4();
        let node = Node::new(bounds, None, false, "base".to_string(), Modals::Base);

        let base = Base::new(node_id);
        self.base = Some(base);
        self.nodes.insert(node_id.clone(), node);
        self.modal = None;
        self.cache.clear();
    }

    pub fn save_revolute(&mut self, mut joint: Revolute) {
        // Ensure the body has a unique name if it's empty
        if joint.name.is_empty() {
            self.counter_revolute += 1;
            joint.name = format!("revolute{}", self.counter_revolute);
        }

        let joint_modal = Modals::Revolute(joint.clone());

        let size = Size::new(100.0, 50.0); //TODO: make width dynamic based on name length
        let top_left = Point::new(
            self.last_graph_cursor_position.x - size.width / 2.0,
            self.last_graph_cursor_position.y - size.height / 2.0,
        );
        let bounds = Rectangle::new(top_left, size);

        let node_id = Uuid::new_v4();
        let node = Node::new(bounds, None, false, joint.name.clone(), joint_modal);

        let joint_id = Uuid::new_v4();
        let revolute = Revolute::new(joint.name.clone(), joint_id);
        let joint = Joint::Revolute(revolute);

        self.joints.insert(joint_id, joint);
        self.nodes.insert(node_id.clone(), node);
        self.modal = None;
        self.cache.clear();
    }

    fn get_snappable_node(&self, cursor: Cursor, nodes: &HashMap<Uuid, Node>) -> Option<Uuid> {
        let mut snap_to = None;
        if let Some(cursor_position) = cursor.position_in(self.graph.bounds) {
            nodes.iter().for_each(|(id, node)| {
                if (cursor_position.x > node.bounds.x
                    && cursor_position.x < node.bounds.x + node.bounds.width)
                    && (cursor_position.y > node.bounds.y
                        && cursor_position.y < node.bounds.y + node.bounds.height)
                {
                    snap_to = Some(id.clone());
                }
            });
        }
        snap_to
    }

    fn window_resized(&mut self, window_size: Size) {
        self.graph.bounds.height = window_size.height;
        self.graph.bounds.width = window_size.width - self.nodebar.bounds.width;
    }
}

fn create_default_node(
    nodes: &mut HashMap<Uuid, Node>,
    node_id: Uuid,
    label: &str,
    node_size: Size,
    home: Point,
    modal: Modals,
) {
    nodes.insert(
        node_id.clone(),
        Node::new(
            Rectangle::new(home.clone(), node_size),
            Some(home.clone()),
            true,
            label.to_string(),
            modal,
        ),
    );
}
fn default_nodes(node_map: NodeBarMap) -> HashMap<Uuid, Node> {
    let mut nodes = HashMap::<Uuid, Node>::new();

    let padding = 15.0;
    let mut count = 1.0;
    let height = 50.0;

    let node_size = Size::new(100.0, height);

    //add base node
    create_default_node(
        &mut nodes,
        node_map.base,
        "+base",
        node_size,
        Point::new(padding, count * padding + (count - 1.0) * height),
        Modals::Base,
    );
    count += 1.0;

    create_default_node(
        &mut nodes,
        node_map.body,
        "+body",
        node_size,
        Point::new(padding, count * padding + (count - 1.0) * height),
        Modals::Body(Body::default()),
    );
    count += 1.0;

    create_default_node(
        &mut nodes,
        node_map.revolute,
        "+revolute",
        node_size,
        Point::new(padding, count * padding + (count - 1.0) * height),
        Modals::Revolute(Revolute::default()),
    );
    count += 1.0;

    nodes
}

async fn load() -> Result<(), String> {
    Ok(())
}

impl Application for IcedTest {
    type Message = Message;
    type Theme = crate::ui::theme::Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Self::Message>) {
        (
            Self::Loading,
            Command::<Message>::batch(vec![
                font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
                Command::perform(load(), Message::Loaded),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("jds")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            IcedTest::Loading => {
                if let Message::Loaded(_) = message {
                    *self = IcedTest::Loaded(AppState::default())
                }
            }
            IcedTest::Loaded(state) => match message {
                Message::FontLoaded(_) => {}
                Message::Loaded(_) => {}
                Message::BodyNameInputChanged(value) => update_body_name(state, &value),
                Message::BodyMassInputChanged(value) => update_body_mass(state, &value),
                Message::BodyCmxInputChanged(value) => update_body_cmx(state, &value),
                Message::BodyCmyInputChanged(value) => update_body_cmy(state, &value),
                Message::BodyCmzInputChanged(value) => update_body_cmz(state, &value),
                Message::BodyIxxInputChanged(value) => update_body_ixx(state, &value),
                Message::BodyIyyInputChanged(value) => update_body_iyy(state, &value),
                Message::BodyIzzInputChanged(value) => update_body_izz(state, &value),
                Message::BodyIxyInputChanged(value) => update_body_ixy(state, &value),
                Message::BodyIxzInputChanged(value) => update_body_ixz(state, &value),
                Message::BodyIyzInputChanged(value) => update_body_iyz(state, &value),
                Message::RevoluteNameInputChanged(value) => update_revolute_name(state, &value),
                Message::LeftButtonPressed(cursor) => state.left_button_pressed(cursor),
                Message::LeftButtonReleased(cursor) => state.left_button_released(cursor),
                Message::RightButtonPressed(cursor) => state.right_button_pressed(cursor),
                Message::RightButtonReleased(cursor) => state.right_button_released(cursor),
                Message::CloseModal => state.modal = None,
                Message::CursorMoved(cursor) => state.cursor_moved(cursor),
                Message::DeletePressed => state.delete_pressed(),
                Message::EnterPressed => state.enter_pressed(),
                Message::SaveBase => state.save_base(),
                Message::SaveBody(modal) => state.save_body(modal),
                Message::SaveRevolute(modal) => state.save_revolute(modal),
                Message::WindowResized(size) => state.window_resized(size),
            },
        }
        Command::none()
    }

    fn view(&self) -> Element<Message, crate::ui::theme::Theme> {
        match self {
            IcedTest::Loading => loading_view(),
            IcedTest::Loaded(state) => loaded_view(state),
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced::event::listen_with(|event, _| match event {
            iced::Event::Window(_, window::Event::Resized { width, height }) => Some(
                Message::WindowResized(Size::new(width as f32, height as f32)),
            ),
            iced::Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => match key {
                keyboard::Key::Named(keyboard::key::Named::Enter) => Some(Message::EnterPressed),
                keyboard::Key::Named(keyboard::key::Named::Delete) => Some(Message::DeletePressed),
                _ => None,
            },
            _ => None,
        })
    }
}
// Helper function to create the loading view
fn loading_view() -> Element<'static, Message, crate::ui::theme::Theme> {
    container(
        text("Loading...")
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(50),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .center_x()
    .into()
}

// Helper function to create the main loaded view
fn loaded_view(state: &AppState) -> Element<Message, crate::ui::theme::Theme> {
    let graph_canvas = GraphCanvas::new(state);
    let graph_container = container(
        Canvas::new(graph_canvas)
            .width(Length::Fill)
            .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill);

    let underlay = Row::new().push(graph_container);

    let overlay = if let Some(active_modal_id) = state.modal {
        if let Some(active_node) = state.nodes.get(&active_modal_id) {
            match &active_node.modal {
                Modals::Base => Some(create_base_modal()),
                Modals::Body(body) => Some(create_body_modal(body)),
                Modals::Revolute(joint) => Some(create_revolute_modal(joint)),
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    };

    modal(underlay, overlay)
        .on_esc(Message::CloseModal)
        .align_y(alignment::Vertical::Center)
        .into()
}

fn create_base_modal() -> Element<'static, Message, crate::ui::theme::Theme> {
    let content = Column::new();
    let footer = Row::new()
        .spacing(10)
        .padding(5)
        .width(Length::Fill)
        .push(
            button("Cancel")
                .width(Length::Fill)
                .on_press(crate::Message::CloseModal),
        )
        .push(
            button("Ok")
                .width(Length::Fill)
                .on_press(crate::Message::SaveBase),
        );

    card("Base Information", content)
        .foot(footer)
        .max_width(500.0)
        .into()
}

fn create_body_modal(body: &Body) -> Element<Message, crate::ui::theme::Theme> {
    let body_clone = body.clone();
    let create_text_input = |label: &str, value: &str, on_input: fn(String) -> Message| {
        Row::new()
            .spacing(10)
            .push(text(label).width(Length::FillPortion(1)))
            .push(
                text_input(label, value)
                    .on_input(on_input)
                    .on_submit(Message::SaveBody(body_clone.clone()))
                    .width(Length::FillPortion(4)),
            )
            .width(Length::Fill)
    };

    let content = Column::new()
        .push(create_text_input(
            "name",
            &body_clone.name,
            crate::Message::BodyNameInputChanged,
        ))
        .push(create_text_input(
            "mass",
            &body_clone.mass.to_string(),
            crate::Message::BodyMassInputChanged,
        ))
        .push(create_text_input(
            "cmx",
            &body_clone.cmx.to_string(),
            crate::Message::BodyCmxInputChanged,
        ))
        .push(create_text_input(
            "cmy",
            &body_clone.cmy.to_string(),
            crate::Message::BodyCmyInputChanged,
        ))
        .push(create_text_input(
            "cmz",
            &body_clone.cmz.to_string(),
            crate::Message::BodyCmzInputChanged,
        ))
        .push(create_text_input(
            "ixx",
            &body_clone.ixx.to_string(),
            crate::Message::BodyIxxInputChanged,
        ))
        .push(create_text_input(
            "iyy",
            &body_clone.iyy.to_string(),
            crate::Message::BodyIyyInputChanged,
        ))
        .push(create_text_input(
            "izz",
            &body_clone.izz.to_string(),
            crate::Message::BodyIzzInputChanged,
        ))
        .push(create_text_input(
            "ixy",
            &body_clone.ixy.to_string(),
            crate::Message::BodyIxyInputChanged,
        ))
        .push(create_text_input(
            "ixz",
            &body_clone.ixz.to_string(),
            crate::Message::BodyIxzInputChanged,
        ))
        .push(create_text_input(
            "iyz",
            &body_clone.iyz.to_string(),
            crate::Message::BodyIyzInputChanged,
        ));

    let footer = Row::new()
        .spacing(10)
        .padding(5)
        .width(Length::Fill)
        .push(
            button("Cancel")
                .width(Length::Fill)
                .on_press(crate::Message::CloseModal),
        )
        .push(
            button("Ok")
                .width(Length::Fill)
                .on_press(crate::Message::SaveBody(body_clone.clone())),
        );

    card("Body Information", content)
        .foot(footer)
        .max_width(500.0)
        .into()
}

fn create_revolute_modal(joint: &Revolute) -> Element<Message, crate::ui::theme::Theme> {
    let joint_clone = joint.clone();
    let content = Column::new().push(
        text_input("name", &joint_clone.name)
            .on_input(|string| crate::Message::RevoluteNameInputChanged(string))
            .on_submit(Message::SaveRevolute(joint_clone.clone())),
    );

    let footer = Row::new()
        .spacing(10)
        .padding(5)
        .width(Length::Fill)
        .push(
            button("Cancel")
                .width(Length::Fill)
                .on_press(crate::Message::CloseModal),
        )
        .push(
            button("Ok")
                .width(Length::Fill)
                .on_press(crate::Message::SaveRevolute(joint_clone.clone())),
        );

    card("Revolute Information", content)
        .foot(footer)
        .max_width(500.0)
        .into()
}

fn update_body_numeric_field<F>(state: &mut AppState, value: &str, update_fn: F)
where
    F: FnOnce(&mut Body, f64),
{
    if let Ok(float_value) = value.parse::<f64>() {
        if let Some(add_body_node) = state.nodes.get_mut(&state.nodebar.map.body) {
            if let Modals::Body(ref mut body) = add_body_node.modal {
                update_fn(body, float_value);
            }
        }
    }
}

fn update_body_name(state: &mut AppState, value: &str) {
    if let Some(add_body_node) = state.nodes.get_mut(&state.nodebar.map.body) {
        if let Modals::Body(ref mut body) = &mut add_body_node.modal {
            body.name = value.to_string();
        }
    }
}

fn update_body_mass(state: &mut AppState, value: &str) {
    update_body_numeric_field(state, value, |body, float_value| {
        body.mass = float_value;
    });
}

fn update_body_cmx(state: &mut AppState, value: &str) {
    update_body_numeric_field(state, value, |body, float_value| {
        body.cmx = float_value;
    });
}

fn update_body_cmy(state: &mut AppState, value: &str) {
    update_body_numeric_field(state, value, |body, float_value| {
        body.cmy = float_value;
    });
}

fn update_body_cmz(state: &mut AppState, value: &str) {
    update_body_numeric_field(state, value, |body, float_value| {
        body.cmz = float_value;
    });
}

fn update_body_ixx(state: &mut AppState, value: &str) {
    update_body_numeric_field(state, value, |body, float_value| {
        body.ixx = float_value;
    });
}

fn update_body_iyy(state: &mut AppState, value: &str) {
    update_body_numeric_field(state, value, |body, float_value| {
        body.iyy = float_value;
    });
}

fn update_body_izz(state: &mut AppState, value: &str) {
    update_body_numeric_field(state, value, |body, float_value| {
        body.izz = float_value;
    });
}

fn update_body_ixy(state: &mut AppState, value: &str) {
    update_body_numeric_field(state, value, |body, float_value| {
        body.ixy = float_value;
    });
}

fn update_body_ixz(state: &mut AppState, value: &str) {
    update_body_numeric_field(state, value, |body, float_value| {
        body.ixz = float_value;
    });
}

fn update_body_iyz(state: &mut AppState, value: &str) {
    update_body_numeric_field(state, value, |body, float_value| {
        body.iyz = float_value;
    });
}

// Helper function to update the revolute name
fn update_revolute_name(state: &mut AppState, value: &str) {
    if let Some(add_joint_node) = state.nodes.get_mut(&state.nodebar.map.revolute) {
        if let Modals::Revolute(ref mut joint_modal) = &mut add_joint_node.modal {
            joint_modal.name = value.to_string();
        }
    }
}
