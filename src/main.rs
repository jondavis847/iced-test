use iced::{
    alignment, font,
    mouse::Cursor,
    widget::{
        button,
        canvas::{Cache, Canvas},
        column, container, text, text_input, Row,
    },
    Application, Command, Element, Length, Point, Settings, Theme,
};

use iced_aw::{card, modal};
//use std::collections::HashMap;

mod canvas;
mod multibody;

use crate::canvas::graph::Graph;
use crate::canvas::node::{ClickedNode, Node};
use crate::canvas::node_bar::NodeBar;
use crate::canvas::GraphCanvas;
use crate::multibody::Body;

fn main() -> iced::Result {
    IcedTest::run(Settings::default())
}

// Define the possible user interactions
#[derive(Debug, Clone)]
enum Message {
    AddBodyNameInputChanged(String),
    CanvasTranslating(Point),
    LeftButtonPressed(Cursor),
    LeftButtonReleased(Cursor),
    CursorMoved(Cursor),
    CloseModal,
    FontLoaded(Result<(), font::Error>),
    Loaded(Result<(), String>),
    SaveBody,
}

#[derive(Debug, Clone, Copy)]
enum NodeType {
    Base,
    Body,
    Revolute,
}

#[derive(Debug)]
enum IcedTest {
    Loading,
    Loaded(AppState),
}

#[derive(Debug)]
struct AppState {
    add_body_name_input: String,
    cache: Cache,
    clicked_node: Option<ClickedNode>,
    bodies: Vec<Body>,
    last_mouse_position: Point,
    modal: Option<NodeType>,
    graph: Graph,
    is_pressed: bool,
    node_bar: NodeBar,
}

impl Default for AppState {
    fn default() -> Self {
        let bodies = Vec::<Body>::new();
        Self {
            add_body_name_input: String::new(),
            bodies: bodies,
            cache: Cache::new(),
            clicked_node: None,
            graph: Graph::default(),
            last_mouse_position: Point::default(),
            modal: None,
            is_pressed: false,
            node_bar: NodeBar::default(),
        }
    }
}

impl AppState {
    pub fn cursor_moved(&mut self, cursor: Cursor) {
        self.last_mouse_position = cursor.position().unwrap();
        self.translate_nodes(cursor);
    }
    pub fn translate_nodes(&mut self, cursor: Cursor) {
        if let Some(cursor_position) = cursor.position() {
            self.node_bar
                .nodes
                .iter_mut()
                .for_each(|node| node.translate_node(cursor_position));
            if let Some(cursor_graph_position) = cursor.position_in(self.graph.bounds) {
                self.bodies
                    .iter_mut()
                    .for_each(|body| body.translate_node(cursor_graph_position));
            }
            self.cache.clear();
        }
    }

    pub fn left_button_pressed(&mut self, cursor: Cursor) {
        // determine if it's pressed on the node bar or canvas
        let clicked_node = &mut self.clicked_node;
        if cursor.is_over(self.node_bar.bounds) {
            self.node_bar.get_clicked_nodes(cursor, clicked_node);
        };
        if let Some(cursor_position) = cursor.position_in(self.graph.bounds) {
            self.last_mouse_position = cursor_position;
            for body in &mut self.bodies {
                let node = &mut body.node;
                node.is_clicked(cursor_position);
                println!("{:?}", node);
                if node.is_clicked {
                    *clicked_node = Some(ClickedNode::new(node.modal, false));
                }
            }
        }
    }

    pub fn left_button_released(&mut self, cursor: Cursor) {
        if let Some(cursor_position) = cursor.position_in(self.graph.bounds) {
            self.last_mouse_position = cursor_position;
        }
        if let Some(clicked_node) = &self.clicked_node {
            if clicked_node.is_nodebar {
                if cursor.is_over(self.graph.bounds) {
                    self.modal = Some(clicked_node.node_type);
                }
            }
            // reset
            self.node_bar.nodes.iter_mut().for_each(|node| node.drop());
            self.bodies.iter_mut().for_each(|body| body.drop());
            self.clicked_node = None;
            self.cache.clear();
        }
    }

    pub fn save_body(&mut self) {
        let body_name = self.add_body_name_input.clone();
        let node = Node::new(self.last_mouse_position, self.graph.zoom, NodeType::Body);
        let body = Body::new(body_name, node);
        self.bodies.push(body);
        self.modal = None;
        self.cache.clear();
    }
}

async fn load() -> Result<(), String> {
    Ok(())
}

impl Application for IcedTest {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self::Loading,
            Command::batch(vec![
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
                Message::AddBodyNameInputChanged(value) => state.add_body_name_input = value,
                Message::LeftButtonPressed(cursor) => state.left_button_pressed(cursor),
                Message::LeftButtonReleased(cursor) => state.left_button_released(cursor),
                Message::CloseModal => state.modal = None,
                Message::CursorMoved(cursor) => state.cursor_moved(cursor),
                Message::SaveBody => state.save_body(),
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

                let overlay = if state.modal.is_some() {
                    match state.modal.unwrap() {
                        NodeType::Base => None,
                        NodeType::Body => Some(
                            card(
                                "Body Information",
                                column![text_input("name", &state.add_body_name_input)
                                    .on_input(Message::AddBodyNameInputChanged)],
                            )
                            .foot(
                                Row::new()
                                    .spacing(10)
                                    .padding(5)
                                    .width(Length::Fill)
                                    .push(
                                        button("Cancel")
                                            .width(Length::Fill)
                                            .on_press(Message::CloseModal),
                                    )
                                    .push(
                                        button("Ok")
                                            .width(Length::Fill)
                                            .on_press(Message::SaveBody),
                                    ),
                            )
                            .on_close(Message::CloseModal)
                            .max_width(500.0),
                        ),
                        NodeType::Revolute => None,
                    }
                } else {
                    None
                };
                modal(underlay, overlay)
                    .on_esc(Message::CloseModal)
                    .align_y(alignment::Vertical::Center)
                    .into()
            }
        }
    }
}
