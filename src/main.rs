use iced::{
    alignment, font,
    widget::{button, column, container, text, text_input, Container, Row},
    Alignment, Application, Command, Element, Length, Settings, Theme,
};

use iced_aw::{card, modal};
//use std::collections::HashMap;

mod graph;
mod multibody;

pub use crate::multibody::Body;
use graph::Graph;

fn main() -> iced::Result {
    IcedTest::run(Settings::default())
}

// Define the possible user interactions
#[derive(Debug, Clone)]
enum Message {
    AddBodyClicked,
    AddBodyNameInputChanged(String),
    CloseAddBodyModal,
    FontLoaded(Result<(), font::Error>),
    Loaded(Result<(), String>),
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
    last_message: Option<Message>,
    show_add_body_modal: bool,
    graph: Graph,
}

impl State {
    fn add_body(&mut self, body: Body) {
        let body_name = body.name.clone();
        self.bodies.push(body);
        self.graph.add_body(body_name)
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            add_body_name_input: String::new(),
            bodies: Vec::<Body>::new(),
            last_message: None,
            show_add_body_modal: false,
            graph: Graph::default(),
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
                Message::AddBodyClicked => state.show_add_body_modal = true,
                Message::AddBodyNameInputChanged(value) => state.add_body_name_input = value,
                Message::CloseAddBodyModal => state.show_add_body_modal = false,
                Message::SaveBody => {
                    let body = Body::new(state.add_body_name_input.clone());
                    let body_name = body.name.clone();
                    state.bodies.push(body);
                    state.graph.add_body(body_name);
                    state.show_add_body_modal = false;
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
                    .graph
                    .canvas
                    .container()
                    .width(Length::FillPortion(15))
                    .height(Length::Fill);

                let button_bar = container(
                    button("Add Body")
                        .width(Length::Fill)
                        .on_press(Message::AddBodyClicked),
                )
                .width(Length::FillPortion(1))
                .height(Length::Fill);

                let underlay_row = Row::new().push(button_bar).push(graph_canvas);

                let underlay = Container::new(underlay_row);

                let overlay = if state.show_add_body_modal {
                    Some(
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
                                        .on_press(Message::CloseAddBodyModal),
                                )
                                .push(button("Ok").width(Length::Fill).on_press(Message::SaveBody)),
                        )
                        .max_width(500.0)
                        //.width(Length::Shrink)
                        .on_close(Message::CloseAddBodyModal),
                    )
                } else {
                    None
                };

                modal(underlay, overlay)
                    .backdrop(Message::CloseAddBodyModal)
                    .on_esc(Message::CloseAddBodyModal)
                    .align_y(alignment::Vertical::Center)
                    .into()
            }
        }
    }
}
