use crate::font::Font;
use crate::ui::modals::Modals;
use crate::ui::canvas::themes::Theme;

use iced::{
    alignment::{Horizontal, Vertical},
    widget::canvas::{path::Path, stroke, Frame, Stroke, Text},
    Color, Point, Rectangle, Vector,
};



#[derive(Debug, Clone)]
pub struct Node {
    pub bounds: Rectangle,
    pub home: Option<Point>,
    pub is_left_clicked: bool,
    pub is_middle_clicked: bool,
    pub is_nodebar: bool,
    pub is_right_clicked: bool,
    pub label: String,
    pub modal: Modals,
}

impl Node {
    pub fn new(
        bounds: Rectangle,
        home: Option<Point>,
        is_nodebar: bool,
        label: String,
        modal: Modals,
    ) -> Self {
        Self {
            bounds: bounds,
            home: home,
            is_left_clicked: false,
            is_middle_clicked: false,
            is_nodebar: is_nodebar,
            is_right_clicked: false,
            label: label,
            modal: modal,
        }
    }

    pub fn draw(&self, frame: &mut Frame, theme: &Theme) {
        let background = Path::rectangle(self.bounds.position(), self.bounds.size());

        let node_border_color;
        if self.is_left_clicked {
            node_border_color = theme.edge_multibody;
        } else {
            if self.is_nodebar {
                node_border_color = theme.dark_border;
            } else {
                node_border_color = theme.edge_multibody;
            }
        }

        let node_background_color = theme.node_background;

        frame.with_save(|frame| {
            frame.stroke(
                &background,
                Stroke {
                    style: stroke::Style::Solid(node_border_color),
                    width: 5.0,
                    ..Stroke::default()
                },
            );
            frame.fill(&background, node_background_color);
            frame.fill_text(Text {
                content: self.label.clone(),
                color: theme.edge_multibody,
                font: Font::MONOSPACE,
                horizontal_alignment: Horizontal::Center,
                position: self.bounds.center(),
                vertical_alignment: Vertical::Center,
                ..Text::default()
            });
        });
    }

    pub fn translate_by(&mut self, graph_translation: Vector) {
        self.bounds.x = self.bounds.x + graph_translation.x;
        self.bounds.y = self.bounds.y + graph_translation.y;
    }

    pub fn translate_to(&mut self, position: Point) {
        self.bounds.x = position.x - self.bounds.width / 2.0;
        self.bounds.y = position.y - self.bounds.height / 2.0;
    }

    pub fn is_clicked(&mut self, cursor_position: Point, mouse_button: &crate::MouseButton) {
        let is_inside = self.bounds.contains(cursor_position);

        match mouse_button {
            crate::MouseButton::Left => self.is_left_clicked = is_inside,
            crate::MouseButton::Right => self.is_right_clicked = is_inside,
            crate::MouseButton::Middle => self.is_middle_clicked = is_inside,
        }
    }

    pub fn drop(&mut self) {
        if let Some(home) = &self.home {
            //send it home
            self.bounds.x = home.x;
            self.bounds.y = home.y;
            self.is_left_clicked = false;
        }
    }
}
