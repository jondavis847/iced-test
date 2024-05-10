use iced::{Color,Point};
use iced::widget::canvas::{Path,stroke::{self,Stroke}};
use uuid::Uuid;
use std::collections::HashMap;
use crate::ui::canvas::node::Node;
use crate::ui::canvas::themes::Theme;

#[derive(Debug, Clone)]
pub enum EdgeConnection {
    Node(Uuid),
    Point(Point),
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub from: EdgeConnection,
    pub to: EdgeConnection,
    pub control: Option<Point>,
}

impl Edge {
    pub fn new(from: EdgeConnection, to: EdgeConnection) -> Self {
        Self {
            from: from,
            to: to,
            control: None,
        }
    }

    pub fn draw(&self, frame: &mut iced::widget::canvas::Frame, nodes: &HashMap<Uuid,Node>, theme: &Theme) {
        let from_point = match self.from {
            EdgeConnection::Node(id) => nodes.get(&id).unwrap().bounds.center(),
            EdgeConnection::Point(point) => point,
        };

        let to_point = match self.to {
            EdgeConnection::Node(id) => nodes.get(&id).unwrap().bounds.center(),
            EdgeConnection::Point(point) => point,
        };

        let control_point = Point::new((from_point.x + to_point.x)/2.0,(from_point.y + to_point.y)/2.0 );

        let path = Path::new(|p| {
            p.move_to(from_point);
            p.quadratic_curve_to(control_point,to_point);
        });

        frame.with_save(|frame| {
            frame.stroke(
                &path,
                Stroke {
                    style: stroke::Style::Solid(theme.edge_multibody),
                    width: 3.0,
                    ..Stroke::default()
                },
            );
        });
    }
}
