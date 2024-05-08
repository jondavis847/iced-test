use iced::{Point, Rectangle, Size};

#[derive(Debug)]
pub struct Graph {
    pub bounds: Rectangle,
    pub zoom: f32,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            bounds: Rectangle::new(Point::new(100.0, 0.0), Size::new(900.0, 1000.0)),
            zoom: 1.0,
        }
    }
}
