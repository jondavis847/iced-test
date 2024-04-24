use iced::{
    alignment, font,
    widget::{button, column, container, text, text_input, Container, Row},
    Application, Command, Element, Length, Point, Settings, Theme,
};

use iced_aw::{card, modal};
//use std::collections::HashMap;

mod canvas;
mod multibody;

use crate::canvas::node::NodeType;
use crate::canvas::Canvas as GraphCanvas;
use crate::multibody::Body;

fn main() -> iced::Result {
    IcedTest::run(Settings::default())
}

// Define the possible user interactions
#[derive(Debug, Clone)]
enum Message {
    AddBodyNameInputChanged(String),
    CanvasTranslating(Point),
    CanvasButtonPressed(Point),
    CloseModal,
    FontLoaded(Result<(), font::Error>),
    Loaded(Result<(), String>),
    NodeAdded(NodeType, Point),
    SaveBody,
}

#[derive(Debug)]
enum IcedTest {
    Loading,
    Loaded(State),
}

#[derive(Debug)]
struct State {
    add_body_name_input: String,
    bodies: Vec<Body>,
    modal: Option<NodeType>,
    canvas: GraphCanvas,
}

impl Default for State {
    fn default() -> Self {
        Self {
            add_body_name_input: String::new(),
            bodies: Vec::<Body>::new(),
            modal: None,
            canvas: GraphCanvas::default(),
        }
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
                    *self = IcedTest::Loaded(State::default())
                }
            }
            IcedTest::Loaded(state) => match message {
                Message::AddBodyNameInputChanged(value) => state.add_body_name_input = value,
                Message::CloseModal => state.modal = None,
                Message::NodeAdded(nodetype,position) => match nodetype {
                    NodeType::Base => state.modal = Some(NodeType::Base),
                    NodeType::Body => {
                        state.modal = Some(NodeType::Body);
                        state.canvas.add_node(state.add_body_name_input.clone(), position, NodeType::Body);
                    }                            
                    NodeType::Revolute => state.modal = Some(NodeType::Revolute),
                },
                Message::SaveBody => {
                    let body = Body::new(state.add_body_name_input.clone());
                    let body_name = body.name.clone();
                    state.bodies.push(body);
                    state.modal = None;
                }
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
                let graph_canvas = state
                    .canvas
                    .container()
                    .width(Length::Fill)
                    .height(Length::Fill);

                let underlay = Row::new().push(graph_canvas);

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
