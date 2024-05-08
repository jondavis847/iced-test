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

use crate::multibody::Body;
use crate::ui::canvas::graph::Graph;
use crate::ui::canvas::node::Node;
use crate::ui::canvas::node_bar::{NodeBar, NodeBarMap};
use crate::ui::canvas::GraphCanvas;
use crate::ui::modals::{BodyModal, Modals};

fn main() -> iced::Result {
    IcedTest::run(Settings::default())
}

// Define the possible user interactions
#[derive(Debug, Clone)]
enum Message {
    AddBodyNameInputChanged(String),
    LeftButtonPressed(Cursor),
    LeftButtonReleased(Cursor),
    CursorMoved(Cursor),
    CloseModal,
    FontLoaded(Result<(), font::Error>),
    Loaded(Result<(), String>),
    SaveBody(BodyModal),
}

#[derive(Debug)]
enum IcedTest {
    Loading,
    Loaded(AppState),
}

#[derive(Debug)]
struct AppState {
    bodies: Vec<Body>,
    cache: Cache,
    clicked_node: Option<Uuid>,
    graph: Graph,
    is_pressed: bool,
    last_graph_cursor_position: Point,
    modal: Option<Uuid>, //uuid of node, modal is owned by node
    nodebar: NodeBar,
    nodes: HashMap<Uuid, Node>,
}

impl Default for AppState {
    fn default() -> Self {
        let nodebar = NodeBar::default();
        let default_nodes = default_nodes(nodebar.clone().map);
        Self {
            bodies: Vec::new(),
            cache: Cache::new(),
            clicked_node: None,
            graph: Graph::default(),
            is_pressed: false,
            last_graph_cursor_position: Point::default(),
            modal: None,
            nodebar: nodebar,
            nodes: default_nodes,
        }
    }
}

impl AppState {
    pub fn get_clicked_node(&mut self, cursor: Cursor) {
        self.clicked_node = None;
        self.nodes.iter_mut().for_each(|(key, node)| {
            if node.is_nodebar {
                // use canvas position
                if let Some(cursor_position) = cursor.position() {
                    node.is_clicked(cursor_position);
                    if node.is_clicked {
                        self.clicked_node = Some(key.clone());
                    }
                }
            } else {
                // use graph position
                if let Some(cursor_position) = cursor.position_in(self.graph.bounds) {
                    node.is_clicked(cursor_position);
                    if node.is_clicked {
                        self.clicked_node = Some(key.clone());
                    }
                }
            }
        });
    }

    pub fn cursor_moved(&mut self, cursor: Cursor) {
        if let Some(clicked_node_id) = self.clicked_node {
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
    }

    pub fn left_button_pressed(&mut self, cursor: Cursor) {
        self.is_pressed = true;
        self.get_clicked_node(cursor);
        if let Some(graph_cursor_position) = cursor.position_in(self.graph.bounds) {
            self.last_graph_cursor_position = graph_cursor_position;
        }
    }

    pub fn left_button_released(&mut self, cursor: Cursor) {
        self.is_pressed = false;
        if let Some(clicked_node_id) = self.clicked_node {
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
        self.clicked_node = None;
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

        let body = Body::new(name, node_id.clone());

        self.bodies.push(body);
        self.nodes.insert(node_id.clone(), node);
        self.modal = None;
        self.cache.clear();
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
    let node_size = Size::new(100.0, 50.0);

    //add base node
    create_default_node(
        &mut nodes,
        node_map.base,
        "+base",
        node_size,
        Point::new(0.0, 0.0),
        Modals::Base,
    );

    create_default_node(
        &mut nodes,
        node_map.body,
        "+body",
        node_size,
        Point::new(0.0, 50.0),
        Modals::Body(BodyModal::new(String::new())),
    );

    create_default_node(
        &mut nodes,
        node_map.revolute,
        "+revolute",
        node_size,
        Point::new(0.0, 100.0),
        Modals::Revolute,
    );

    nodes
}

async fn load() -> Result<(), String> {
    Ok(())
}

impl Application for IcedTest {
    type Message = Message;
    type Theme = iced::Theme;
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

    fn theme(&self) -> Self::Theme {
        Self::Theme::Oxocarbon
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
                Message::CloseModal => state.modal = None,
                Message::CursorMoved(cursor) => state.cursor_moved(cursor),
                Message::SaveBody(modal) => state.save_body(modal),
                _ => {}
            },
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
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
