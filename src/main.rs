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
    window, Application, Command, Element, Length, Settings, Size, Subscription,
};

use iced_aw::{card, modal};
use std::time::{Duration, Instant};

mod multibody;
mod ui;

use crate::multibody::MultibodyTrait;
use crate::ui::canvas::graph::{Graph, GraphMessage};
use crate::ui::canvas::nodebar::{Nodebar, NodebarMessage};
use crate::ui::canvas::GraphCanvas;
use crate::ui::dummies::{DummyBase, DummyBody, DummyComponent, DummyRevolute, DummyTrait};
use crate::ui::modals::ActiveModal;

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
    TabPressed,
    FontLoaded(Result<(), font::Error>),
    Loaded(Result<(), String>),
    SaveComponent,
    WindowResized(Size),
}

#[derive(Debug)]
enum IcedTest {
    Loading,
    Loaded(AppState),
}

#[derive(Debug)]
struct AppState {    
    cache: Cache,    
    counter_body: usize,
    counter_revolute: usize,    
    graph: Graph,    
    left_clicked_time_1: Option<Instant>,
    left_clicked_time_2: Option<Instant>,    
    modal: Option<ActiveModal>,
    nodebar: Nodebar,    
    theme: crate::ui::theme::Theme,
}

impl Default for AppState {
    fn default() -> Self {
        Self {            
            cache: Cache::new(),            
            counter_body: 0,
            counter_revolute: 0,            
            left_clicked_time_1: None,
            left_clicked_time_2: None,            
            graph: Graph::default(),            
            modal: None,
            nodebar: Nodebar::default(),            
            theme: crate::ui::theme::Theme::ORANGE,            
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
    Nothing,
}

impl AppState {
    pub fn close_modal(&mut self) -> Command<Message> {
        self.modal = None;
        Command::none()
    }
    
    pub fn cursor_moved(&mut self, cursor: Cursor) -> Command<Message> {
        let nodebar_redraw = self.nodebar.cursor_moved(cursor);
        let graph_redraw = self.graph.cursor_moved(cursor);

        // don't need to redraw just because mouse is moving
        if nodebar_redraw || graph_redraw {
            self.cache.clear();
        }
        Command::none()
    }

    pub fn delete_pressed(&mut self) -> Command<Message> {
        self.graph.delete_pressed();
        //self.nodebar.delete_pressed(); // no need for this, maybe ever?
        self.cache.clear();
        Command::none()
    }

    pub fn enter_pressed(&mut self) -> Command<Message> {
        self.save_component();
        Command::none()
    }

    pub fn left_button_pressed(&mut self, cursor: Cursor) -> Command<Message> {
        self.left_clicked_time_1 = self.left_clicked_time_2;
        self.left_clicked_time_2 = Some(Instant::now());

        self.nodebar.left_button_pressed(cursor);
        self.graph.left_button_pressed(cursor);        
        self.cache.clear();
        Command::none()
    }

    pub fn left_button_released(&mut self, cursor: Cursor) -> Command<Message> {
        // Determine the type of mouse button release event
        let release_event = match (self.left_clicked_time_1, self.left_clicked_time_2) {
            (Some(clicked_time_1), Some(clicked_time_2)) => {
                let clicked_elapsed_time_1 = clicked_time_1.elapsed();
                let clicked_elapsed_time_2 = clicked_time_2.elapsed();

                if clicked_elapsed_time_1 <= Duration::from_millis(500) {
                    MouseButtonReleaseEvents::DoubleClick
                } else if clicked_elapsed_time_2 <= Duration::from_millis(300) {
                    MouseButtonReleaseEvents::SingleClick
                } else {
                    MouseButtonReleaseEvents::Held
                }
            }
            (None, Some(clicked_time_2)) => {
                if clicked_time_2.elapsed() <= Duration::from_millis(200) {
                    MouseButtonReleaseEvents::SingleClick
                } else {
                    MouseButtonReleaseEvents::Held
                }
            }
            _ => MouseButtonReleaseEvents::Nothing,
        };        
        if let Some(NodebarMessage::NewComponent(id)) = self.nodebar.left_button_released(&release_event) {
            // Only create a new component if the mouse is over the graph
            if cursor.is_over(self.graph.bounds) {
                if self.nodebar.components.contains_key(&id) {
                    self.modal = Some(ActiveModal::new(id, None));
                }
            }
        }
        if let Some(GraphMessage::EditComponent(id)) = self.graph.left_button_released(&release_event, cursor) {            
            if let Some(component) = self.graph.components.get(&id) {                
                let active_modal = ActiveModal::new(*component.get_dummy_id(), Some(id));                
        
                if let Some(dummy) = self.nodebar.components.get_mut(&active_modal.dummy_component_id) {                    
                    if let Some(component_id) = active_modal.graph_component_id {
                        // Editing an existing component, populate values
                        dummy.inherit_from(&component_id, &self.graph);                        
                    }
        
                    self.modal = Some(active_modal);
                }
            }
        }
        
        self.cache.clear();
        Command::none()
    }

    pub fn right_button_pressed(&mut self, cursor: Cursor) -> Command<Message> {
        self.nodebar.right_button_pressed(cursor);
        self.graph.right_button_pressed(cursor);    
        Command::none()    
    }

    pub fn right_button_released(&mut self, cursor: Cursor) -> Command<Message> {
        self.graph.right_button_released(cursor);
        self.nodebar.right_button_released(cursor);
        self.cache.clear();
        Command::none()
    }

    pub fn save_component(&mut self) -> Command<Message> {
        // early return
        let modal = match self.modal {
            Some(ref modal) => modal,
            None => return Command::none(),
        };

        // early return
        let dummy_component = match self.nodebar.components.get_mut(&modal.dummy_component_id) {
            Some(component) => component,
            None => return Command::none(),
        };

        // Ensure the body has a unique name if it's empty
        if dummy_component.get_name().is_empty() {
            let name = match dummy_component {
                DummyComponent::Base(_) => "base".to_string(),
                DummyComponent::Body(_) => {
                    self.counter_body += 1;
                    format!("body{}", self.counter_body)
                }
                DummyComponent::Revolute(_) => {
                    self.counter_revolute += 1;
                    format!("revolute{}", self.counter_revolute)
                }
            };
            dummy_component.set_name(&name);
        }
        match modal.graph_component_id {
            Some(id) => self.graph.edit_component(&dummy_component,&id),
            None => self.graph.save_component(&dummy_component),
        }
        

        // Clear the modal and cache
        dummy_component.clear();
        self.modal = None;
        self.cache.clear();
        Command::none()
    }

    fn tab_pressed(&mut self) -> Command<Message> {
        if self.modal.is_some() {
            Command::none()
            //iced::widget::focus_next() // not working right now
        } else {
            Command::none()
        }
    }

    fn update_body_name(&mut self, value: &str) -> Command<Message> {
        if let Some(dummy_component) = self.nodebar.components.get_mut(&self.nodebar.map.body) {
            if let DummyComponent::Body(dummy_body) = dummy_component {
                dummy_body.set_name(value);
            } else {
                // Handle error: must be the dummy body
                eprintln!("Error: Component is not a DummyBody");
            }
        }
        Command::none()
    }
    
    fn update_body_mass(&mut self, value: &str) -> Command<Message> {
        if let Some(dummy_component) = self.nodebar.components.get_mut(&self.nodebar.map.body) {
            if let DummyComponent::Body(dummy_body) = dummy_component {
                dummy_body.mass = value.to_string();
            } else {
                // Handle error: must be the dummy body
                eprintln!("Error: Component is not a DummyBody");
            }
        }
        Command::none()
    }
    
    fn update_body_cmx(&mut self, value: &str) -> Command<Message> {
        if let Some(dummy_component) = self.nodebar.components.get_mut(&self.nodebar.map.body) {
            if let DummyComponent::Body(dummy_body) = dummy_component {
                dummy_body.cmx = value.to_string();
            } else {
                // Handle error: must be the dummy body
                eprintln!("Error: Component is not a DummyBody");
            }
        }
        Command::none()
    }
    
    fn update_body_cmy(&mut self, value: &str) -> Command<Message> {
        if let Some(dummy_component) = self.nodebar.components.get_mut(&self.nodebar.map.body) {
            if let DummyComponent::Body(dummy_body) = dummy_component {
                dummy_body.cmy = value.to_string();
            } else {
                // Handle error: must be the dummy body
                eprintln!("Error: Component is not a DummyBody");
            }
        }
        Command::none()
    }
    
    fn update_body_cmz(&mut self, value: &str) -> Command<Message> {
        if let Some(dummy_component) = self.nodebar.components.get_mut(&self.nodebar.map.body) {
            if let DummyComponent::Body(dummy_body) = dummy_component {
                dummy_body.cmz = value.to_string();
            } else {
                // Handle error: must be the dummy body
                eprintln!("Error: Component is not a DummyBody");
            }
        }
        Command::none()
    }
    
    fn update_body_ixx(&mut self, value: &str) -> Command<Message> {
        if let Some(dummy_component) = self.nodebar.components.get_mut(&self.nodebar.map.body) {
            if let DummyComponent::Body(dummy_body) = dummy_component {
                dummy_body.ixx = value.to_string();
            } else {
                // Handle error: must be the dummy body
                eprintln!("Error: Component is not a DummyBody");
            }
        }
        Command::none()
    }
    
    fn update_body_iyy(&mut self, value: &str) -> Command<Message> {
        if let Some(dummy_component) = self.nodebar.components.get_mut(&self.nodebar.map.body) {
            if let DummyComponent::Body(dummy_body) = dummy_component {
                dummy_body.iyy = value.to_string();
            } else {
                // Handle error: must be the dummy body
                eprintln!("Error: Component is not a DummyBody");
            }
        }
        Command::none()
    }
    
    fn update_body_izz(&mut self, value: &str) -> Command<Message> {
        if let Some(dummy_component) = self.nodebar.components.get_mut(&self.nodebar.map.body) {
            if let DummyComponent::Body(dummy_body) = dummy_component {
                dummy_body.izz = value.to_string();
            } else {
                // Handle error: must be the dummy body
                eprintln!("Error: Component is not a DummyBody");
            }
        }
        Command::none()
    }
    
    fn update_body_ixy(&mut self, value: &str) -> Command<Message> {
        if let Some(dummy_component) = self.nodebar.components.get_mut(&self.nodebar.map.body) {
            if let DummyComponent::Body(dummy_body) = dummy_component {
                dummy_body.ixy = value.to_string();
            } else {
                // Handle error: must be the dummy body
                eprintln!("Error: Component is not a DummyBody");
            }
        }
        Command::none()
    }
    
    fn update_body_ixz(&mut self, value: &str) -> Command<Message> {
        if let Some(dummy_component) = self.nodebar.components.get_mut(&self.nodebar.map.body) {
            if let DummyComponent::Body(dummy_body) = dummy_component {
                dummy_body.ixz = value.to_string();
            } else {
                // Handle error: must be the dummy body
                eprintln!("Error: Component is not a DummyBody");
            }
        }
        Command::none()
    }
    
    fn update_body_iyz(&mut self, value: &str) -> Command<Message> {
        if let Some(dummy_component) = self.nodebar.components.get_mut(&self.nodebar.map.body) {
            if let DummyComponent::Body(dummy_body) = dummy_component {
                dummy_body.iyz = value.to_string();
            } else {
                // Handle error: must be the dummy body
                eprintln!("Error: Component is not a DummyBody");
            }
        }
        Command::none()
    }
    
    // Helper function to update the revolute name
    fn update_revolute_name(&mut self, value: &str) -> Command<Message> {
        if let Some(dummy_component) = self.nodebar.components.get_mut(&self.nodebar.map.revolute) {
            if let DummyComponent::Revolute(dummy_revolute) = dummy_component {
                dummy_revolute.set_name(value);
            } else {
                // Handle error: must be the dummy revolute
                eprintln!("Error: Component is not a DummyRevolute");
            }
        }  
        Command::none()  
    }
    

    fn window_resized(&mut self, window_size: Size) -> Command<Message> {
        let graph_size = Size::new(
            window_size.width - self.nodebar.bounds.width,
            window_size.height,
        );
        self.graph.window_resized(graph_size);
        let nodebar_size = Size::new(
            self.nodebar.bounds.width,
            window_size.height,
        );
        self.nodebar.window_resized(nodebar_size);
        self.cache.clear();
        Command::none()
    }
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
                    *self = IcedTest::Loaded(AppState::default());                    
                }
                Command::none()
            }
            IcedTest::Loaded(state) => match message {
                Message::FontLoaded(_) => Command::none(),
                Message::Loaded(_) => Command::none(),
                Message::BodyNameInputChanged(value) => state.update_body_name(&value),
                Message::BodyMassInputChanged(value) => state.update_body_mass( &value),
                Message::BodyCmxInputChanged(value) => state.update_body_cmx( &value),
                Message::BodyCmyInputChanged(value) => state.update_body_cmy( &value),
                Message::BodyCmzInputChanged(value) => state.update_body_cmz( &value),
                Message::BodyIxxInputChanged(value) => state.update_body_ixx( &value),
                Message::BodyIyyInputChanged(value) => state.update_body_iyy( &value),
                Message::BodyIzzInputChanged(value) => state.update_body_izz( &value),
                Message::BodyIxyInputChanged(value) => state.update_body_ixy( &value),
                Message::BodyIxzInputChanged(value) => state.update_body_ixz( &value),
                Message::BodyIyzInputChanged(value) => state.update_body_iyz( &value),
                Message::RevoluteNameInputChanged(value) => state.update_revolute_name( &value),
                Message::LeftButtonPressed(cursor) => state.left_button_pressed(cursor),
                Message::LeftButtonReleased(cursor) => state.left_button_released(cursor),
                Message::RightButtonPressed(cursor) => state.right_button_pressed(cursor),
                Message::RightButtonReleased(cursor) => state.right_button_released(cursor),
                Message::CloseModal => state.close_modal(),
                Message::CursorMoved(cursor) => state.cursor_moved(cursor),
                Message::DeletePressed => state.delete_pressed(),
                Message::EnterPressed => state.enter_pressed(),
                Message::TabPressed => state.tab_pressed(),
                Message::SaveComponent => state.save_component(),
                Message::WindowResized(size) => state.window_resized(size),
            },
        }        
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
                keyboard::Key::Named(keyboard::key::Named::Tab) => Some(Message::TabPressed),
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

    let overlay = 
    // if we have an ActiveModal
    if let Some(active_modal) = state.modal {
        // and there's a DummyComponent for that ActiveModal
        if let Some(dummy) = state
            .nodebar
            .components
            .get(&active_modal.dummy_component_id)
        {
            match dummy {
                DummyComponent::Base(base) => Some(create_base_modal(base)),
                DummyComponent::Body(body) => Some(create_body_modal(body)),
                DummyComponent::Revolute(joint) => Some(create_revolute_modal(joint)),                
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

fn create_base_modal(_base: &DummyBase) -> Element<'static, Message, crate::ui::theme::Theme> {
    let content = Column::new();
    let footer = Row::new()
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
                .on_press(Message::SaveComponent),
        );

    card("Base Information", content)
        .foot(footer)
        .max_width(500.0)
        .into()
}

fn create_body_modal(body: &DummyBody) -> Element<Message, crate::ui::theme::Theme> {    
    let create_text_input = |label: &str, value: &str, on_input: fn(String) -> Message| {
        Row::new()
            .spacing(10)
            .push(text(label).width(Length::FillPortion(1)))
            .push(
                text_input(label, value)
                    .on_input(on_input)
                    .on_submit(Message::SaveComponent)
                    .width(Length::FillPortion(4)),
            )
            .width(Length::Fill)
    };

    let content = Column::new()
        .push(create_text_input(
            "name",
            &body.name,
            Message::BodyNameInputChanged,
        ))
        .push(create_text_input(
            "mass",
            &body.mass,
            Message::BodyMassInputChanged,
        ))
        .push(create_text_input(
            "cmx",
            &body.cmx,
            Message::BodyCmxInputChanged,
        ))
        .push(create_text_input(
            "cmy",
            &body.cmy,
            Message::BodyCmyInputChanged,
        ))
        .push(create_text_input(
            "cmz",
            &body.cmz,
            Message::BodyCmzInputChanged,
        ))
        .push(create_text_input(
            "ixx",
            &body.ixx,
            Message::BodyIxxInputChanged,
        ))
        .push(create_text_input(
            "iyy",
            &body.iyy,
            Message::BodyIyyInputChanged,
        ))
        .push(create_text_input(
            "izz",
            &body.izz,
            Message::BodyIzzInputChanged,
        ))
        .push(create_text_input(
            "ixy",
            &body.ixy,
            Message::BodyIxyInputChanged,
        ))
        .push(create_text_input(
            "ixz",
            &body.ixz,
            Message::BodyIxzInputChanged,
        ))
        .push(create_text_input(
            "iyz",
            &body.iyz,
            Message::BodyIyzInputChanged,
        ));
        

    let footer = Row::new()
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
                .on_press(Message::SaveComponent),
        );

    card("Body Information", content)
        .foot(footer)
        .max_width(500.0)
        .into()
}

fn create_revolute_modal(joint: &DummyRevolute) -> Element<Message, crate::ui::theme::Theme> {    
    let content = Column::new().push(
        text_input("name", &joint.name)
            .on_input(|string| crate::Message::RevoluteNameInputChanged(string))
            .on_submit(Message::SaveComponent),
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
                .on_press(crate::Message::SaveComponent),
        );

    card("Revolute Information", content)
        .foot(footer)
        .max_width(500.0)
        .into()
}

