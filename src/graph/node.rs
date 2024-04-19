use crate::font::Font;
use iced::{
    alignment::{Horizontal,Vertical},
    widget::canvas::{path::Path, Frame, Text},
    Point, Size, Theme, Vector,
};

#[derive(Debug,Clone)]
pub struct Node {
    label: String,    
    pub rendered_position: Point,
    graph_position: Vector,
    size: Size,    
}

impl Node {
    pub fn new(label: String, rendered_position: Point, graph_origin: Point) -> Self {        
        let height = 50.0;
        let width = 100.0;
        let graph_position = rendered_position - graph_origin;
        Self {
            label: label,            
            rendered_position: rendered_position,
            graph_position: graph_position,
            size: Size::new(width, height),            
        }
    }

    pub fn draw(&self, frame: &mut Frame, theme: &Theme) {  
        //frame.translate(frame.center() - Point::ORIGIN);      
        let node_center = Vector::new(self.size.width/2.0,self.size.height/2.0);
        let translation_vec = self.rendered_position;
        let translation_pt = Point::new(translation_vec.x,translation_vec.y);// - node_center;

        let palette = theme.extended_palette();        
        let background = Path::rectangle(translation_pt, self.size);
        frame.with_save(|frame| {
            
            frame.fill(&background, palette.primary.strong.color);        
            frame.fill_text(Text {
                content: self.label.clone(),
                color: palette.primary.strong.text,            
                font: Font::MONOSPACE,
                horizontal_alignment: Horizontal::Center,
                position: translation_pt+node_center,
                vertical_alignment: Vertical::Center,
                ..Text::default()
            });        
        });
    }
}
