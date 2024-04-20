use iced::{
    mouse::{self, Cursor},
    widget::{
        canvas::{
            self,
            event::{Event, Status},
            Cache, Canvas as IcedCanvas, Geometry, Path, Stroke,
        },
        container, Container,
    },
    Length, Point, Rectangle, Renderer, Theme, Vector,
};

use crate::graph::node::Node;
use crate::Message;

#[derive(Debug, Default)]
pub struct Canvas {    
    pub cache: Cache, 
    pub nodes: Vec<Node>,
    pub last_mouse_position: Point,
    pub translation: Vector,
    pub zoom_scaling: f32,    
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

    pub fn add_node(&mut self, label: String, rendered_position: Vector) {
        let node = Node::new(label, rendered_position, self.translation);
        self.nodes.push(node);
        self.request_redraw();
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear();
    }

    pub fn translate_nodes(&mut self) {
        for node in &mut self.nodes {
            node.translate_rendered_position(self.translation);
        }
    }
}

#[derive(Debug, Default)]
pub struct CanvasState {
    pub is_pressed: bool,    
    pub was_pressed: bool,    
}

impl canvas::Program<Message> for Canvas {
    type State = CanvasState;

    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (Status, Option<Message>) {
        let Some(cursor_position) = cursor.position_over(bounds) else {
            return (Status::Ignored, None);
        };

        match event {
            Event::Mouse(mouse_event) => {
                match mouse_event {
                    mouse::Event::ButtonPressed(mouse::Button::Left) => {
                        state.is_pressed = true;                        
                        (Status::Captured, Some(Message::CanvasButtonPressed(cursor_position)))
                    }
                    mouse::Event::ButtonReleased(mouse::Button::Left) => {
                        state.is_pressed = false;
                        (Status::Captured, None)
                    },                   
                    mouse::Event::CursorMoved { position } => {
                        if state.is_pressed {                            
                            (Status::Captured, Some(Message::CanvasTranslating(position)))
                        } else {
                            (Status::Captured, None)   
                        }
                        
                    }                    
                    _ => (Status::Captured, None),
                }                
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
            for node in &self.nodes {
                frame.with_save(|frame| {
                    //frame.with_clip(bounds, |frame| {
                        node.draw(frame, theme);
                    //});
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
