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
    Length, Point, Rectangle, Renderer, Size, Theme, Vector,
};
pub mod graph;
pub mod node;
pub mod node_bar;

use crate::canvas::graph::Graph;
use crate::canvas::node::{Node,NodeType};
use crate::canvas::node_bar::NodeBar;
use crate::Message;

#[derive(Debug, Default)]
pub struct Canvas {
    pub cache: Cache,
    pub node_bar: NodeBar,
    pub graph: Graph,    
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

    pub fn add_node(&mut self, label: String, rendered_position: Point, nodetype: NodeType) {
        let node = Node::new(label, rendered_position, self.zoom_scaling, nodetype);
        self.request_redraw();
    }

    pub fn request_redraw(&mut self) {
        self.cache.clear();
    }
}

#[derive(Debug)]
pub struct CanvasState {
    pub is_pressed: bool,
    pub was_pressed: bool,
    pub nodebar_nodes: Vec<Node>,     
    pub graph_nodes: Vec<Node>,    
    pub translation: Vector,
    pub last_mouse_position: Point,
    pub grabbed_node: Option<GrabbedNode>,
}

impl CanvasState {
    pub fn translate_nodes(&mut self) {
        for node in &mut self.graph_nodes {
            node.translate_rendered_position(self.translation);
        }
    }
}

impl Default for CanvasState {
    fn default() ->Self {
        let mut nodes = Vec::<Node>::new();
        nodes.push(Node::new("Base".into(), Point::new(0.0, 0.0), 1.0, NodeType::Base));
        nodes.push(Node::new("Body".into(), Point::new(0.0, 50.0), 1.0, NodeType::Body));
        nodes.push(Node::new("Revolute".into(), Point::new(0.0, 100.0), 1.0, NodeType::Revolute));
        Self {            
            is_pressed: false,
            was_pressed: false,
            nodebar_nodes: nodes,            
            graph_nodes: Vec::new(),            
            translation: Vector::default(),
            last_mouse_position: Point::default(),
            grabbed_node: None,
        }
    }
}

#[derive(Debug)]
struct GrabbedNode {
    pub is_nodebar: bool,    
    pub index: usize,
    pub nodetype: NodeType,
}
impl GrabbedNode {
    fn new(is_nodebar: bool, index: usize, nodetype: NodeType) -> Self {
        Self{is_nodebar,index,nodetype}
    }
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
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::ButtonPressed(mouse::Button::Left) => {
                    state.is_pressed = true;

                    // determine if it's pressed on the node bar or canvas
                    if cursor.position_over(self.node_bar.bounds).is_some() {
                        self.node_bar.get_clicked_node(state,cursor);
                    };
                    if cursor.position_over(self.graph.bounds).is_some() {
                        state.last_mouse_position = cursor_position;
                        self.graph.get_clicked_node(state, cursor);
                    };
                    (
                        Status::Captured,
                        Some(Message::CanvasButtonPressed(cursor_position)),
                    )
                }
                mouse::Event::ButtonReleased(mouse::Button::Left) => {
                    let mut message = None;
                    state.is_pressed = false;
                    if let Some(node) = &state.grabbed_node {                        
                        if node.is_nodebar {
                            // Add a node
                            message = Some(Message::NodeAdded(node.nodetype,cursor_position));
                            // return nodebar node back to nodebar
                            state.nodebar_nodes[node.index].bounds.x = 0.0;
                            state.nodebar_nodes[node.index].bounds.y = 50.0 * node.index as f32;
                            self.cache.clear();                            
                        }
                    }
                    state.grabbed_node = None;
                    (Status::Captured, message)
                }
                mouse::Event::CursorMoved { position } => {
                    if state.is_pressed {
                        //add logic to save the clicked node instead of rerunning this loop
                        for node in &mut state.nodebar_nodes {
                            if node.is_clicked {                                
                                node.translate_to(position);
                                //println!("{:?}", node.bounds.position());
                                self.cache.clear();
                                return (Status::Captured, None)
                            }                            
                        }

                       // let new_translation = position - state.last_mouse_position;
                        //state.translation = state.translation + new_translation;
                        //state.last_mouse_position = position;
                        //state.translate_nodes();
                        //self.cache.clear();
                    }
                    (Status::Captured, None)
                }
                _ => (Status::Captured, None),
            },
            _ => (Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let all_content = self.cache.draw(renderer, bounds.size(), |frame| {
            // the node bar
            self.node_bar.draw(state,frame, theme);
            // the graph canvas
            self.graph.draw(frame, theme, state);

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
