use iced::{
    mouse::{self, Cursor},
    widget::{
        canvas::{
            self,
            event::{Event, Status},
            Cache, Canvas as IcedCanvas, Geometry, Path, Stroke,
        },
        container,Container
    },
    Element, Length, Point, Rectangle, Renderer, Theme, Vector,
};

use crate::graph::node::Node;
use crate::Message;

#[derive(Debug, Default)]
pub struct Canvas {
    pub nodes: Vec<Node>,
    pub cache: Cache,
    pub origin: Point,
}

impl Canvas {
    pub fn container(&self) -> Container<Message>
    where
        Message: Clone,
    {
        container(
            IcedCanvas::new(self)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        
    }

    pub fn add_node(&mut self, label: String, rendered_position: Point) {
        let node = Node::new(label, rendered_position, self.origin);
        self.nodes.push(node);
        self.request_redraw();
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear();
    }
}

#[derive(Debug)]
pub struct State {
    pub zoom_scaling: f32,
    pub center: Point,
    pub last_mouse_position: Point,
    pub translation: Vector,
    pub is_pressed: bool,
    pub was_pressed: bool,
}

impl State {}
impl Default for State {
    fn default() -> Self {
        Self {
            zoom_scaling: 1.0,
            center: Point::default(),
            last_mouse_position: Point::default(),
            translation: Vector::default(),
            is_pressed: false,
            was_pressed: false,
        }
    }
}

impl canvas::Program<Message> for Canvas {
    type State = State;

    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (Status, Option<Message>) {
        let Some(cursor_position) = cursor.position_in(bounds) else {
            return (Status::Ignored, None);
        };

        match event {
            Event::Mouse(mouse_event) => {
                match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        state.is_pressed = true;
                        state.last_mouse_position = cursor_position;
                        println!("{:?}", cursor_position);
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => state.is_pressed = false,
                    mouse::Event::CursorMoved { position } => {
                        if state.is_pressed {
                            let new_translation = position - state.last_mouse_position;
                            state.translation = state.translation + new_translation.into();
                            state.last_mouse_position = position;
                        }
                    }
                    _ => (),
                };
                (Status::Captured, None)
            }
            _ => (Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let content = self.cache.draw(renderer, bounds.size(), |frame| {
            let nodes = self.nodes.clone(); //TODO: try not to clone?
            for node in nodes {
                frame.with_save(|frame| {
                    frame.with_clip(bounds, |frame| {
                        node.draw(frame, theme);
                    });
                });
            }

            // canvas border
            frame.stroke(
                &Path::rectangle(Point::ORIGIN, frame.size()),
                Stroke::default().with_width(2.0),
            );
        });
        vec![content]
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if cursor.is_over(bounds) {
            if state.is_pressed {
                mouse::Interaction::Grabbing
            } else {
                mouse::Interaction::Grab
            }
        } else {
            mouse::Interaction::default()
        }
    }
}
