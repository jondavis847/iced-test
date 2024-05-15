use iced::{application, Color};

macro_rules! color {
    ($red:expr, $green:expr, $blue:expr) => {
        Color::from_rgb(
            $red as f32 / 255.0,
            $green as f32 / 255.0,
            $blue as f32 / 255.0,
        )
    };
}

#[derive(Debug)]
pub struct Theme {
    pub background: Color,
    pub node_background: Color,
    pub text: Color,
    pub greyed: Color,
    pub border: Color,
    pub shadow: Color,
    pub primary: Color,
    pub highlight: Color,
}

impl Theme {
    pub const ORANGE: Self = Self {
        background: color!(64, 61, 57),
        node_background: color!(51, 49, 46),
        text: color!(255, 252, 242),
        greyed: color!(204, 197, 185),
        border: color!(37, 36, 34),
        shadow: color!(37, 36, 34),
        primary: color! (255, 140, 0),
        highlight: color!(255,255,0),
    };
}

impl Default for Theme {
    fn default() -> Self {
        Self::ORANGE
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum Application {
    #[default]
    Default,
}

impl iced::application::StyleSheet for Theme {
    type Style = Application;

    fn appearance(&self, style: &Self::Style) -> iced::application::Appearance {
        match style {
            Application::Default => application::Appearance {
                background_color: self.background.into(),
                text_color: self.text,
            },
        }
    }
}

impl iced::widget::container::StyleSheet for Theme {
    type Style = Application;
    fn appearance(&self, style: &Self::Style) -> iced::widget::container::Appearance {
        match style {
            Application::Default => {
                let mut border = iced::Border::with_radius(2.0);
                border.color = self.border;
                border.width = 2.0;

                let shadow = iced::Shadow {
                    color: self.shadow,
                    offset: iced::Vector::new(3.0, 3.0),
                    blur_radius: 4.0,
                };

                iced::widget::container::Appearance {
                    text_color: None,
                    background: Some(self.background.into()),
                    border: border,
                    shadow: shadow,
                }
            }
        }
    }
}

impl iced::widget::text::StyleSheet for Theme {
    type Style = Application;

    fn appearance(&self, _style: Self::Style) -> iced::widget::text::Appearance {
        iced::widget::text::Appearance {
            color: Some(self.text),
        }
    }
}

impl iced::widget::button::StyleSheet for Theme {
    type Style = Application;

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(self.background)),
            text_color: self.text,
            ..Default::default()
        }
    }
}

impl iced_aw::style::card::StyleSheet for Theme {
    type Style = Application;

    fn active(&self, _style: &Self::Style) -> iced_aw::style::card::Appearance {
        iced_aw::style::card::Appearance {
            background: iced::Background::Color(self.background),
            head_background: iced::Background::Color(self.primary),
            ..Default::default()
        }
    }
}

impl iced::widget::text_input::StyleSheet for Theme {
    type Style = Application;

    fn active(&self, _style: &Self::Style) -> iced::widget::text_input::Appearance {
        let border = iced::Border{            
            color: self.border,
            ..Default::default()
        };
        
        iced::widget::text_input::Appearance {
            background: iced::Background::Color(self.background),            
            border: border,
            icon_color: self.shadow,
        }
    }

    fn focused(&self, _style: &Self::Style) -> iced::widget::text_input::Appearance {
        let border = iced::Border{            
            color: self.border,
            ..Default::default()
        };
        
        iced::widget::text_input::Appearance {
            background: iced::Background::Color(self.background),            
            border: border,
            icon_color: self.shadow,
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        self.greyed
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        self.text
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        self.greyed
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        self.text
    }

    fn disabled(&self, _style: &Self::Style) -> iced::widget::text_input::Appearance {
        let border = iced::Border{            
            color: self.border,
            ..Default::default()
        };
        
        iced::widget::text_input::Appearance {
            background: iced::Background::Color(self.background),            
            border: border,
            icon_color: self.shadow,
        }
    }


}

impl iced_aw::style::modal::StyleSheet for Theme {
    type Style = ();
    fn active(&self, _style: &Self::Style) -> iced_aw::style::modal::Appearance {
        iced_aw::style::modal::Appearance {
            background: self.background.into(),
        }
    }
}
