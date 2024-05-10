use iced::Color;

#[derive(Debug)]
pub struct Theme {
    pub background: Color,
    pub dark_border: Color, 
    pub edge_actuator: Color,
    pub edge_multibody: Color,
    pub edge_sensor: Color,
    pub node_background: Color,
    pub node_hover: Color,    
    pub node_selected: Color,    
    pub plot_colors: [Color;5]
}

#[derive(Debug)]
pub enum Themes {
    Dracula(Theme),
    Cyberpunk(Theme),
}

impl Themes {
    pub fn cyberpunk() -> Theme {
        Theme {
            background: Color::from_rgb8(68,71,90), 
            dark_border: Color::from_rgb8(15, 15, 15),
            edge_actuator: Color::from_rgb8(255,87,34), //pink
            edge_multibody: Color::from_rgb8(255,184,108), //orange
            edge_sensor: Color::from_rgb8(0,255,255), //blue
            node_background: Color::from_rgb8(40,42,54),
            node_hover: Color::from_rgb8(0,0,34),
            node_selected: Color::from_rgb8(255,255,85), //yellow
            plot_colors: [
                Color::from_rgb8(255,87,34), //orange
                Color::from_rgb8(153,102,255), //purple
                Color::from_rgb8(255,0,255), //pink
                Color::from_rgb8(255,255,85), //yellow
                Color::from_rgb8(255,0,0), //red
            ],
        }
    }

    pub fn dracula() -> Theme {
        Theme {
            background: Color::from_rgb8(40,42,54), 
            dark_border: Color::from_rgb8(26, 28, 38),
            edge_actuator: Color::from_rgb8(255,121,198), //pink
            edge_multibody: Color::from_rgb8(255,184,108), //orange
            edge_sensor: Color::from_rgb8(139,233,253), //blue
            node_background: Color::from_rgb8(33,34,44),
            node_hover: Color::from_rgb8(46,48,62),
            node_selected: Color::from_rgb8(241,250,140), //yellow
            plot_colors: [
                Color::from_rgb8(255,184,108), //orange
                Color::from_rgb8(189,147,249), //purple
                Color::from_rgb8(255,121,198), //pink
                Color::from_rgb8(241,250,140), //yellow
                Color::from_rgb8(255,85,85), //red
            ],
        }
    }
}