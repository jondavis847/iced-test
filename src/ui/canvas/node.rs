use crate::font::Font;
use crate::ui::modals::Modals;

use iced::{
    alignment::{Horizontal, Vertical},
    widget::canvas::{path::Path, stroke, Frame, Stroke, Text},
    Color, Point, Rectangle, Theme, Vector,
};

#[derive(Debug, Clone)]
pub struct Node {
    pub bounds: Rectangle,    
    pub home: Option<Point>,
    pub is_clicked:bool,
    pub is_nodebar: bool,
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
            is_clicked: false,
            is_nodebar: is_nodebar,            
            label: label,
            modal: modal,
        }
    }

    pub fn draw(&self, frame: &mut Frame, theme: &Theme) {
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
                content: self.label.clone(),
                color: palette.primary.strong.text,
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

    pub fn is_clicked(&mut self, cursor_position: Point) {
        if self.bounds.contains(cursor_position) {
            self.is_clicked = true;            
        } else {
            self.is_clicked = false;
        }
    }

    pub fn drop(&mut self) {
        if let Some(home) = &self.home {            
            //send it home
            self.bounds.x = home.x;
            self.bounds.y = home.y;            
        }
    }
}
