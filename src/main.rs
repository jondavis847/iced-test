#![windows_subsystem = "windows"]

use iced::{
    alignment, font,
    mouse::Cursor,
    widget::{
        button,
        canvas::{Cache, Canvas},
        container, text, text_input, Column, Row,
    },
    Application, Command, Element, Length, Point, Rectangle, Settings, Size,
};
use std::collections::HashMap;
use uuid::Uuid;

use iced_aw::{card, modal};

mod multibody;
mod ui;

use crate::multibody::{Base, Body};
use crate::ui::canvas::edge::{Edge, EdgeConnection};
use crate::ui::canvas::graph::Graph;
use crate::ui::canvas::node::Node;
use crate::ui::canvas::node_bar::{NodeBar, NodeBarMap};
use crate::ui::canvas::GraphCanvas;
use crate::ui::modals::{BodyModal, Modals, RevoluteModal};

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.antialiasing = true;
    IcedTest::run(settings)
}

// Define the possible user interactions
#[derive(Debug, Clone)]
enum Message {
    AddBodyNameInputChanged(String),
    LeftButtonPressed(Cursor),
    LeftButtonReleased(Cursor),
    RightButtonPressed(Cursor),
    RightButtonReleased(Cursor),
    CursorMoved(Cursor),
    CloseModal,
    FontLoaded(Result<(), font::Error>),
    Loaded(Result<(), String>),
    SaveBase,
    SaveBody(BodyModal),
}

#[derive(Debug)]
enum IcedTest {
    Loading,
    Loaded(AppState),
}

#[derive(Debug)]
struct AppState {
    base: Option<Base>,
    bodies: HashMap<Uuid, Body>,
    cache: Cache,
    //connections: HashMap<Uuid,Connection>,
    left_clicked_node: Option<Uuid>,
    right_clicked_node: Option<Uuid>,
    middle_clicked_node: Option<Uuid>,
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
            cache: Cache::new(),
            left_clicked_node: None,
            right_clicked_node: None,
            middle_clicked_node: None,
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
                }
                self.cache.clear();
            }
        }
    }
    pub fn left_button_pressed(&mut self, cursor: Cursor) {
        self.is_pressed = true;
        self.get_clicked_node(cursor, &MouseButton::Left);
        if let Some(graph_cursor_position) = cursor.position_in(self.graph.bounds) {
            self.last_graph_cursor_position = graph_cursor_position;
        }
    }

    pub fn right_button_pressed(&mut self, cursor: Cursor) {
        self.get_clicked_node(cursor, &MouseButton::Right);
        if let Some(graph_cursor_position) = cursor.position_in(self.graph.bounds) {
            self.last_graph_cursor_position = graph_cursor_position;
        }
    }

    pub fn right_button_released(&mut self, cursor: Cursor) {
        // are we dragging an edge?
        if let Some(edge_id) = self.current_edge {
            // is there a node close enough to snap to?
            if let Some(snappable_node) = self.get_snappable_node(cursor, &self.nodes) {
                if let Some(active_edge) = self.edges.get_mut(&edge_id) {
                    active_edge.to = EdgeConnection::Node(snappable_node);
                }
            } else {
                // nothing to connect to, drop the edge
                self.edges.remove(&edge_id);
            }
            self.current_edge = None;
        }
        self.right_clicked_node = None;
        self.cache.clear();
    }

    pub fn left_button_released(&mut self, cursor: Cursor) {
        self.is_pressed = false;
        if let Some(clicked_node_id) = self.left_clicked_node {
            if let Some(clicked_node) = self.nodes.get_mut(&clicked_node_id) {
                if clicked_node.is_nodebar {
                    if let Some(cursor_position) = cursor.position_in(self.graph.bounds) {
                        self.last_graph_cursor_position = cursor_position;
                    }
                    self.modal = Some(clicked_node_id);
                }
                clicked_node.drop();
            }
        }
        // reset
        self.left_clicked_node = None;
        self.cache.clear();
    }

    pub fn save_body(&mut self, modal: BodyModal) {
        let name = modal.name.clone();
        let body_modal = Modals::Body(modal);

        let size = Size::new(100.0, 50.0); //TODO: make width dynamic based on name length
        let top_left = Point::new(
            self.last_graph_cursor_position.x - size.width / 2.0,
            self.last_graph_cursor_position.y - size.height / 2.0,
        );
        let bounds = Rectangle::new(top_left, size);

        let node_id = Uuid::new_v4();
        let node = Node::new(bounds, None, false, name.clone(), body_modal);

        let body_id = Uuid::new_v4();
        let body = Body::new(name, node_id.clone());

        self.bodies.insert(body_id, body);
        self.nodes.insert(node_id.clone(), node);
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
        Point::new(padding, count*padding + (count-1.0) * height),
        Modals::Base,
    );
    count += 1.0;

    create_default_node(
        &mut nodes,
        node_map.body,
        "+body",
        node_size,
        Point::new(padding, count*padding + (count-1.0) * height),
        Modals::Body(BodyModal::new(String::new())),
    );
    count += 1.0;

    create_default_node(
        &mut nodes,
        node_map.revolute,
        "+revolute",
        node_size,
        Point::new(padding, count*padding + (count-1.0) * height),
        Modals::Revolute(RevoluteModal::new(String::new())),
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
                Message::AddBodyNameInputChanged(value) => {
                    let add_body_node = state.nodes.get_mut(&state.nodebar.map.body);
                    if let Some(add_body_node) = add_body_node {
                        if let Modals::Body(ref mut body_modal) = &mut add_body_node.modal {
                            body_modal.name = value.clone();
                        }
                    }
                }
                Message::LeftButtonPressed(cursor) => state.left_button_pressed(cursor),
                Message::LeftButtonReleased(cursor) => state.left_button_released(cursor),
                Message::RightButtonPressed(cursor) => state.right_button_pressed(cursor),
                Message::RightButtonReleased(cursor) => state.right_button_released(cursor),
                Message::CloseModal => state.modal = None,
                Message::CursorMoved(cursor) => state.cursor_moved(cursor),
                Message::SaveBase => state.save_base(),
                Message::SaveBody(modal) => state.save_body(modal),
                _ => {}
            },
        }
        Command::none()
    }

    fn view(&self) -> Element<Message, crate::ui::theme::Theme> {
        match self {
            IcedTest::Loading => container(
                text("Loading...")
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(50),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .into(),
            IcedTest::Loaded(state) => {
                let graph_canvas = GraphCanvas::new(state);
                let graph_container = container(
                    Canvas::new(graph_canvas)
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .width(Length::Fill)
                .height(Length::Fill);

                let underlay = Row::new().push(graph_container);

                let overlay = match state.modal {
                    Some(active_modal_id) => {
                        state
                            .nodes
                            .get(&active_modal_id)
                            .and_then(|active_node| match &active_node.modal {
                                Modals::Base => {
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

                                    Some(
                                        card("Body Information", content)
                                            .foot(footer)
                                            .max_width(500.0),
                                    )
                                }
                                Modals::Body(body) => {
                                    let body_clone = body.clone();
                                    let content = Column::new().push(
                                        text_input("name", &body_clone.name).on_input(|string| {
                                            crate::Message::AddBodyNameInputChanged(string)
                                        }),
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
                                        .push(button("Ok").width(Length::Fill).on_press(
                                            crate::Message::SaveBody(body_clone.clone()),
                                        ));

                                    Some(
                                        card("Body Information", content)
                                            .foot(footer)
                                            .max_width(500.0),
                                    )
                                }
                                _ => None,
                            })
                    }
                    _ => None,
                };

                modal(underlay, overlay)
                    .on_esc(Message::CloseModal)
                    .align_y(alignment::Vertical::Center)
                    .into()
            }
        }
    }
}
