use crate::imode_gui::Rect;
use crate::imode_gui::Color;

#[derive(Debug, Clone)]
pub struct Style {
    pub padding: Padding,
    pub spacing: Spacing,
    pub auto_wrap: bool,
    pub text_styles: TextStyles,
    pub button: ButtonStyle,
    pub drag_point: Color,
    pub clear_color: Color
}

impl Default for Style {
    fn default() -> Self {
        Self {
            padding: Default::default(),
            spacing: Default::default(),
            auto_wrap: true,
            text_styles: Default::default(),
            button: Default::default(),
            drag_point: Color::Rgb(220, 220, 220),
            clear_color: Color::Rgb(27, 27, 27)
        }
    }
}


#[derive(Debug, Clone)]
pub struct TextStyles {
    pub small: TextStyle,
    pub body: TextStyle,
    pub button: TextStyle,
    pub heading: TextStyle,
}


#[derive(Debug, Clone)]
pub struct TextStyle {
    pub pixel_size: i32,
    pub font_name: String,
}

impl Default for TextStyles {
    fn default() -> Self {
        Self {
            small: TextStyle { pixel_size: 14, font_name: "Consolas".to_string()},
            body: TextStyle { pixel_size: 16, font_name: "Consolas".to_string() },
            button: TextStyle { pixel_size: 16, font_name: "Consolas".to_string() },
            heading: TextStyle { pixel_size: 22, font_name: "Consolas".to_string()},
        }
    }
}


/// Space between widget content and their border
#[derive(Debug, Clone, Copy)]
pub struct Padding {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}

impl Default for Padding {
    fn default() -> Self {
        Self {
            left: 4,
            right: 4,
            top: 4,
            bottom: 4
        }
    }
}

impl Padding {
    pub fn set(&mut self, val: i32) {
        self.left = val;
        self.right = val;
        self.top = val;
        self.bottom = val;
    }

    pub fn x(&self) -> i32 {
        self.left + self.right
    }

    pub fn y(&self) -> i32 {
        self.top + self.bottom
    }
}


/// Space between ui widgets
#[derive(Debug, Clone, Copy)]
pub struct Spacing {
    pub x: i32,
    pub y: i32,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            x: 4,
            y: 4
        }
    }
}

impl Spacing {
    pub fn set(&mut self, val: i32) {
        self.x = val;
        self.y = val;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BorderRadius {
    HeightRelative(f64),
    Fixed(i32) // in pixels
}

impl BorderRadius {

    pub fn get(&self, rect: Rect) -> f64 {
        match self {
            Self::Fixed(pixels) => *pixels as f64,
            Self::HeightRelative(portion) => rect.h as f64 * *portion
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct ButtonStyle {
    pub color: Color,
    pub hover_color: Color,
    pub active_color: Color,
    pub text_color: Color,
    pub radius: BorderRadius,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            color: Color::Rgb(60, 60, 60),
            hover_color: Color::Rgb(70, 70, 70),
            active_color: Color::Rgb(200, 200, 200),
            text_color: Color::Rgb(10, 10, 10),
            radius: BorderRadius::HeightRelative(0.33),
        }
    }
}
