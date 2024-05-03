use crate::font::Font;

use crate::NodeType;
use iced::{
    alignment::{Horizontal, Vertical},
    mouse::Cursor,
    widget::canvas::{path::Path, stroke, Frame, Stroke, Text},
    Color, Point, Rectangle, Size, Theme, Vector,
};

#[derive(Debug, Clone)]
pub struct Node {
    pub bounds: Rectangle,
    pub is_clicked: bool,
    pub is_nodebar: bool,
    pub modal: NodeType,
}

impl Node {
    pub fn new(position: Point, scale: f32, modal: NodeType) -> Self {
        let height = scale * 50.0;
        let width = scale * 100.0;
        let top_left = Point::new(position.x-width/2.0, position.y-height/2.0);
        let size = Size::new(width, height);
        Self {
            bounds: Rectangle::new(top_left, size),
            is_clicked: false,
            is_nodebar: false,
            modal: modal,
        }
    }

    pub fn draw(&self, frame: &mut Frame, theme: &Theme, label: &str) {
        let palette = theme.extended_palette();
        let background = Path::rectangle(self.bounds.position(), self.bounds.size());
        frame.with_save(|frame| {
            frame.stroke(
                &background,
                Stroke {
                    style: stroke::Style::Solid(Color::BLACK),
                    width: 1.0,
                    ..Stroke::default()
                },
            );
            frame.fill(&background, palette.primary.strong.color);
            frame.fill_text(Text {
                content: label.into(),
                color: palette.primary.strong.text,
                font: Font::MONOSPACE,
                horizontal_alignment: Horizontal::Center,
                position: self.bounds.center(),
                vertical_alignment: Vertical::Center,
                ..Text::default()
            });
        });
    }

    pub fn translate_rendered_position(&mut self, canvas_translation: Vector) {
        self.bounds.x = self.bounds.x + canvas_translation.x;
        self.bounds.y = self.bounds.y + canvas_translation.y;
    }

    pub fn translate_to(&mut self, position: Point) {
        if self.is_clicked {
            self.bounds.x = position.x - self.bounds.width / 2.0;
            self.bounds.y = position.y - self.bounds.height / 2.0;
        }
    }

    pub fn is_clicked(&mut self, cursor_position: Point) {       
        println!("{:?}.{:?}", cursor_position,self.bounds) ;
        if self.bounds.contains(cursor_position) {
            self.is_clicked = true;            
        } else {
            self.is_clicked = false;
        }
    }
}

#[derive(Debug)]
pub struct ClickedNode {
    pub node_type: NodeType,
    pub is_nodebar: bool,
}

impl ClickedNode {
    pub fn new(node_type: NodeType, is_nodebar: bool) -> Self {
        Self {
            node_type,
            is_nodebar,
        }
    }
}
