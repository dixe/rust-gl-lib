use crate::imode_gui::Color;

#[derive(Debug, Clone)]
pub struct Style {
    pub padding: Padding,
    pub spacing: Spacing,
    pub auto_wrap: bool,
    pub text_styles: TextStyles,
    pub button: ButtonStyle
}

impl Default for Style {
    fn default() -> Self {
        Self {
            padding: Default::default(),
            spacing: Default::default(),
            auto_wrap: true,
            text_styles: Default::default(),
            button: Default::default(),
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
            small: TextStyle { pixel_size: 12, font_name: "Consolas".to_string()},
            body: TextStyle { pixel_size: 20, font_name: "Consolas".to_string() },
            button: TextStyle { pixel_size: 20, font_name: "Consolas".to_string() },
            heading: TextStyle { pixel_size: 40, font_name: "Consolas".to_string()},
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
pub struct ButtonStyle {
    pub color: Color,
    pub hover_color: Color,
    pub active_color: Color,
    pub text_color: Color,

}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            color: Color::Rgb(109, 156, 116),
            hover_color: Color::Rgb(114, 214, 126),
            active_color: Color::Rgb(111, 135, 114),
            text_color: Color::Rgb(0, 0, 0),
        }
    }
}