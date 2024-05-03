use iced::{
    mouse::Cursor,
    widget::canvas::{Frame, Path, Stroke},
    Point, Rectangle, Size, Theme,
};

use crate::canvas::node::{ClickedNode, Node};
use crate::NodeType;

#[derive(Debug, Clone)]
pub struct NodeBarNode {
    label: &'static str,
    home: Point,
    pub node: Node,
}

impl NodeBarNode {
    pub fn new(label: &'static str, home: Point, node: Node) -> Self {
        Self { label, home, node }
    }

    pub fn draw(&self, frame: &mut Frame, theme: &Theme) {
        self.node.draw(frame, theme, self.label);
    }

    pub fn translate_node(&mut self, position: Point) {
        self.node.translate_to(position);
    }

    pub fn drop(&mut self) {
        //send it home to the nodebar
        self.node.bounds.x = self.home.x - self.node.bounds.width/2.0;
        self.node.bounds.y = self.home.y - self.node.bounds.height/2.0;
        self.node.is_clicked = false;
    }
}

#[derive(Debug)]
pub struct NodeBar {
    pub bounds: Rectangle,
    pub nodes: Vec<NodeBarNode>,
}

impl Default for NodeBar {
    fn default() -> Self {
        let mut nodes = Vec::new();
        let home = Point::new(50.0,25.0);
        nodes.push(NodeBarNode::new(
            "+base",
            home,
            Node::new(home, 1.0, NodeType::Base),
        ));
        let home = Point::new(50.0,75.0);
        nodes.push(NodeBarNode::new(
            "+body",
            home,
            Node::new(home, 1.0, NodeType::Body),
        ));
        let home = Point::new(50.0,125.0);
        nodes.push(NodeBarNode::new(
            "+revolute",
            home,
            Node::new(home, 1.0, NodeType::Revolute),
        ));
        
        Self {
            bounds: Rectangle::new(Point::new(0.0, 0.0), Size::new(100.0, 1000.0)),
            nodes: nodes,
        }
    }
}

impl NodeBar {
    pub fn draw(&self, frame: &mut Frame, theme: &Theme) {
        // border
        frame.stroke(
            &Path::rectangle(
                Point::ORIGIN,
                Size::new(self.bounds.width, frame.size().height),
            ),
            Stroke::default().with_width(2.0),
        );
        for node in &self.nodes {
            node.draw(frame, &theme);
        }
    }

    pub fn get_clicked_nodes(&mut self, cursor: Cursor, clicked_node: &mut Option<ClickedNode>) {
        for node_bar_node in &mut self.nodes {
            let node = &mut node_bar_node.node;
            node.is_clicked(cursor.position().unwrap());
            if node.is_clicked {
                *clicked_node = Some(ClickedNode::new(node.modal, true));
            }
        }
    }
}
