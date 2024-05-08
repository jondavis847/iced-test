use iced::{
    mouse::{self, Cursor},
    widget::canvas::{
        self,
        event::{Event, Status},
        Geometry, Path, Stroke,
    },
    Point, Rectangle, Renderer, Theme,
};
pub mod graph;
pub mod node;
pub mod node_bar;

use crate::Message;

#[derive(Debug)]
pub struct GraphCanvas<'a> {
    app_state: &'a crate::AppState,
}

impl<'a> GraphCanvas<'a> {
    pub fn new(app_state: &'a crate::AppState) -> Self {
        Self {
            app_state: app_state,
        }
    }
}

#[derive(Debug)]
pub struct CanvasState {}

impl Default for CanvasState {
    fn default() -> Self {
        Self {}
    }
}

impl<'a> canvas::Program<Message> for GraphCanvas<'a> {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (Status, Option<Message>) {
        if !cursor.is_over(bounds) {
            return (Status::Ignored, None);
        };

        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    (Status::Captured, Some(Message::LeftButtonPressed(cursor)))
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    (Status::Captured, Some(Message::LeftButtonReleased(cursor)))
                }
                mouse::Event::CursorMoved { position: _ } => {
                    (Status::Captured, Some(Message::CursorMoved(cursor)))
                }
                _ => (Status::Captured, None),
            },
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
        let all_content = self.app_state.cache.draw(renderer, bounds.size(), |frame| {
            // create nodes that are not clipped first (nodebar)
            self.app_state.nodes.iter().for_each(|(_, node)| {
                if node.is_nodebar {
                    node.draw(frame, theme)
                }
            });

            // create nodes that clipped (graph)
            frame.with_clip(self.app_state.graph.bounds, |frame| {
                self.app_state.nodes.iter().for_each(|(_, node)| {
                    if !node.is_nodebar {
                        node.draw(frame, theme)
                    }
                });
            });

            // canvas border
            frame.stroke(
                &Path::rectangle(Point::ORIGIN, frame.size()),
                Stroke::default().with_width(2.0),
            );
        });
        vec![all_content]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if cursor.is_over(bounds) {
            if self.app_state.is_pressed {
                mouse::Interaction::Grabbing
            } else {
                mouse::Interaction::Grab
            }
        } else {
            mouse::Interaction::default()
        }
    }
}
