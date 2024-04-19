pub mod canvas;
pub mod node;
pub mod edge;

use iced::Point;
use canvas::Canvas as GraphCanvas; // not to be confused with iced Canvas

#[derive(Debug,Default)]
pub struct Graph {    
    pub canvas: GraphCanvas,    
}

impl Graph {
    pub fn add_body(&mut self, label: String ) {
        //let node = GraphNode::new(label, self.canvas.state.last_mouse_position, self.canvas.state.translation);                
        self.canvas.add_node(label, Point::new(0.0,0.0));                
    }
}

