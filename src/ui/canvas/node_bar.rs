use uuid::Uuid;

use iced::{    
    Point, Rectangle, Size
};

#[derive(Debug,Clone)]
pub struct NodeBar {
    pub bounds: Rectangle, 
    pub map: NodeBarMap,       
}

impl Default for NodeBar {
    fn default() -> Self {        
        Self {
            bounds: Rectangle::new(Point::new(0.0, 0.0), Size::new(130.0, 1000.0)),  
            map: NodeBarMap::default(),          
        }
    }
}

#[derive(Debug,Clone)]
pub struct NodeBarMap {
    pub base: Uuid,
    pub body: Uuid,
    pub revolute: Uuid,
}

impl Default for NodeBarMap {
    fn default() -> Self {
        Self {
            base: Uuid::new_v4(),
            body: Uuid::new_v4(),
            revolute: Uuid::new_v4(),
        }
    }
}