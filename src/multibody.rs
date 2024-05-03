use crate::canvas::node::Node;
use iced::{widget::canvas::Frame, Point, Theme};

#[derive(Debug, Clone)]
pub struct Body {
    pub name: String,
    pub node: Node,
}

impl Body {
    pub fn new(name: String, node: Node) -> Self {
        Body { name, node }
    }

    pub fn draw(&self, frame: &mut Frame, theme: &Theme) {
        self.node.draw(frame, theme, self.name.as_str());
    }

    pub fn translate_node(&mut self, position: Point) {
        self.node.translate_to(position);
    }

    pub fn drop(&mut self) {
        self.node.is_clicked = false;
    }
}

//enum Joint {
//    Floating,
//    Prismatic,
//    Revolute,
//    Spherical,
//}
