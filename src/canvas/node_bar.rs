use iced::{
    Color,
    mouse::Cursor,
    widget::canvas::{Frame, Path, Stroke},
    Point, Rectangle, Size, Theme,
};

use crate::canvas::{CanvasState,GrabbedNode};
use crate::canvas::node::Node;
use crate::Message;
#[derive(Debug)]
pub struct NodeBar {        
    pub bounds: Rectangle,
}

impl Default for NodeBar {
    fn default() -> Self {
        Self {            
            bounds: Rectangle::new(Point::new(0.0,0.0), Size::new(100.0,1000.0)),            
        }
    }
}

impl NodeBar {
    pub fn draw(&self, state: &CanvasState, frame: &mut Frame, theme: &Theme) {        
        // border
        frame.stroke(
            &Path::rectangle(Point::ORIGIN, Size::new(self.bounds.width, frame.size().height)),
            Stroke::default().with_width(2.0),
        );

        // draw each node
        for node in &state.nodebar_nodes {
            frame.with_save(|frame| {
                node.draw(frame, &theme);
            });
        }
    }

    pub fn get_clicked_node(&self, state: &mut CanvasState, cursor: Cursor) {
        for i in 0..state.nodebar_nodes.len() {        
            state.nodebar_nodes[i].is_clicked(cursor);
            if state.nodebar_nodes[i].is_clicked {
                state.grabbed_node = Some(GrabbedNode::new(true,i,state.nodebar_nodes[i].nodetype));                
            }
        }
    }
}

