use iced::Point;
use iced::widget::canvas::{Path,stroke::{self,Stroke}};
use uuid::Uuid;
use std::collections::HashMap;
use crate::ui::canvas::graph::GraphNode;
use crate::ui::theme::Theme;

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

    pub fn draw(&self, frame: &mut iced::widget::canvas::Frame, nodes: &HashMap<Uuid,GraphNode>, theme: &Theme) {
        let from_point = match self.from {
            EdgeConnection::Node(id) => nodes.get(&id).unwrap().node.bounds.center(),
            EdgeConnection::Point(point) => point,
        };
        
        let to_point = match self.to {
            EdgeConnection::Node(id) => nodes.get(&id).unwrap().node.bounds.center(),
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
                    style: stroke::Style::Solid(theme.primary),
                    width: 3.0,
                    ..Stroke::default()
                },
            );
        });

        // Calculate the direction vector
        let direction = Point::new(to_point.x - from_point.x, to_point.y - from_point.y);
        let length = (direction.x.powi(2) + direction.y.powi(2)).sqrt();
        let unit_direction = Point::new(direction.x / length, direction.y / length);

        // Define the arrowhead size
        let arrowhead_length = 100.0;
        let arrowhead_width = 5.0;

        // Calculate the points of the arrowhead
        let arrow_point1 = Point::new(
            to_point.x - arrowhead_length * unit_direction.x + arrowhead_width * unit_direction.y,
            to_point.y - arrowhead_length * unit_direction.y - arrowhead_width * unit_direction.x,
        );
        let arrow_point2 = Point::new(
            to_point.x - arrowhead_length * unit_direction.x - arrowhead_width * unit_direction.y,
            to_point.y - arrowhead_length * unit_direction.y + arrowhead_width * unit_direction.x,
        );

        // Draw the arrowhead
        let arrow_path = Path::new(|p| {
            p.move_to(to_point);
            p.line_to(arrow_point1);
            p.line_to(arrow_point2);
            p.close();
        });

        frame.fill(
            &arrow_path,
            theme.primary,
        );
    }
}
