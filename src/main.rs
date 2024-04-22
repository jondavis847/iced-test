use iced::{
    alignment, font,
    widget::{button, column, container, text, text_input, Container, Row},
    Application, Command, Element, Length, Point, Settings, Theme,
};

use iced_aw::{card, modal};
//use std::collections::HashMap;

mod canvas;
mod multibody;

use crate::canvas::Canvas as GraphCanvas;
use crate::multibody::Body;

fn main() -> iced::Result {
    IcedTest::run(Settings::default())
}

// Define the possible user interactions
#[derive(Debug, Clone)]
enum Message {
    AddBodyClicked,
    AddBodyNameInputChanged(String),
    CanvasTranslating(Point),
    CanvasButtonPressed(Point),
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
    show_add_body_modal: bool,
    canvas: GraphCanvas,
}

impl Default for State {
    fn default() -> Self {
        Self {
            add_body_name_input: String::new(),
            bodies: Vec::<Body>::new(),
            show_add_body_modal: false,
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
                Message::AddBodyClicked => state.show_add_body_modal = true,
                Message::AddBodyNameInputChanged(value) => state.add_body_name_input = value,
                Message::CloseAddBodyModal => state.show_add_body_modal = false,
                Message::SaveBody => {
                    let body = Body::new(state.add_body_name_input.clone());
                    let body_name = body.name.clone();
                    state.bodies.push(body);
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
                    .canvas
                    .container()
                    .width(Length::Fill)
                    .height(Length::Fill);

                let underlay = Row::new().push(graph_canvas);

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
