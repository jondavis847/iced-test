use iced::{
    widget::canvas::Frame,
    mouse::Cursor,
    Rectangle, Theme
};
use crate::canvas::{CanvasState,GrabbedNode};

#[derive(Debug,Default)]
pub struct Graph {
    pub bounds: Rectangle
}

impl Graph {
    pub fn get_clicked_node(&self, state: &mut CanvasState, cursor: Cursor) {
        for i in 0..state.graph_nodes.len()-1 {        
            state.graph_nodes[i].is_clicked(cursor);
            if state.graph_nodes[i].is_clicked {
                state.grabbed_node = Some(GrabbedNode::new(false,i));
            }
        }
        //logic for multiple nodes clicked
    }

    pub fn draw(&self, frame:&mut Frame, theme: &Theme, state: &CanvasState) {
         // add nodes to the canvas
         for node in &state.graph_nodes {
            frame.with_save(|frame| {
                frame.with_clip(self.bounds, |frame| {
                    node.draw(frame, theme);
                });
            });
        }
    }
}