use crate::sdl_gui::layout::*;
use crate::sdl_gui::components::base::*;
use crate::sdl_gui::layout::attributes::{self, Length, LengthConstraint, Attributes, Attribute, AlignmentY, AlignmentX};
use crate::text_rendering::{ text_renderer::TextRenderer };
use crate::gl;
use crate::sdl_gui::layout::node::Node;
use std::fmt;
use num;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    X,
    Y
}

pub trait Element<Message> : fmt::Debug where Message: fmt::Debug {

    fn name(&self) -> String;

    fn height_children(&self) -> i32 {
        0
    }

    fn width_children(&self) -> i32 {
        0
    }

    fn distribution_dir(&self) -> Direction {
        Direction::X
    }

    fn attributes(&self) -> &Attributes;


    fn attributes_mut(&mut self) -> &mut Attributes;

    fn create_component(&self, _gl: &gl::Gl, _comp_base: ComponentBase) -> Option<Component<Message>> {
        None
    }

    fn content_height(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32;

    fn final_height(&self, available_space: &RealizedSize, text_renderer: &TextRenderer, on_fill: OnFill) -> f32 {

        let attribs = self.attributes();
        let mut h = match attribs.height {
            Length::Px(px) => {
                px as f32
            },
            Length::FitContent => {
                let ch = self.content_height(available_space, text_renderer);
                ch
            },
            _ => match on_fill {
                OnFill::Expand => available_space.height,
                OnFill::Shrink => self.content_height(available_space, text_renderer),
            }
        };

        h += attribs.padding.top + attribs.padding.bottom;


        let min = attribs.height_constraint.min();
        let max = attribs.height_constraint.max(available_space.height);

        let fh = f32::min(max, f32::max(min, h));
        fh

    }

    fn pop_children_front(&mut self) -> Option<Node<Message>>;

    fn content_width(&self, available_space: &RealizedSize, text_renderer: &TextRenderer) -> f32;

    fn final_width(&self, available_space: &RealizedSize, text_renderer: &TextRenderer, on_fill: OnFill) -> f32 {

        let w = match self.attributes().width {
            Length::Px(px) => {
                px as f32
            },
            Length::FitContent => {
                self.content_width(available_space, text_renderer)
            },
            _ => match on_fill {
                OnFill::Expand => available_space.width,
                OnFill::Shrink => self.content_width(available_space, text_renderer),
            }
        };


        let attribs = self.attributes();

        let min = attribs.width_constraint.min();
        let max = attribs.width_constraint.max(available_space.width);

        f32::min(max, f32::max(min, w))

    }

    fn contrainted_width(&self, available_space: &RealizedSize) -> f32 {

        let w = match self.attributes().width {
            Length::Px(px) => {
                px as f32
            },
            _ => available_space.width
        };

        self.bounded_width(w, available_space)
    }


    fn bounded_width(&self, width: f32, available_space: &RealizedSize) -> f32 {

        let attribs = self.attributes();

        let min = attribs.width_constraint.min();
        let max = attribs.width_constraint.max(available_space.width);

        f32::min(max, f32::max(min, width))

    }

    fn width(self, w: Length) -> Self where Self: Sized {
        self.add_attribute(Attribute::Width(w))
    }

    fn height(self, h: Length) -> Self where Self: Sized {
        self.add_attribute(Attribute::Height(h))
    }

    fn max_width<T: num::NumCast>(self, max: T) -> Self where Self: Sized {
        self.add_attribute(Attribute::WidthConstraint(LengthConstraint::Max(num::cast(max).unwrap())))
    }

    fn min_width<T: num::NumCast>(self, min: T) -> Self where Self: Sized {
        self.add_attribute(Attribute::WidthConstraint(LengthConstraint::Min(num::cast(min).unwrap())))
    }

    fn max_height<T: num::NumCast>(self, max: T) -> Self where Self: Sized {
        self.add_attribute(Attribute::HeightConstraint(LengthConstraint::Max(num::cast(max).unwrap())))
    }

    fn min_height<T: num::NumCast>(self, min: T) -> Self where Self: Sized {
        self.add_attribute(Attribute::HeightConstraint(LengthConstraint::Max(num::cast(min).unwrap())))
    }

    fn padding<T: num::NumCast>(self, p: T) -> Self where Self: Sized {
        self.add_attribute(Attribute::Padding(num::cast(p).unwrap()))
    }

    fn padding_bottom<T: num::NumCast> (self, p: T) -> Self where Self: Sized {
        let padding = attributes::Padding {
            left: 0.0,
            right: 0.0,
            top: 0.0,
            bottom: num::cast(p).unwrap()
        };
        self.add_attribute(Attribute::PaddingEach(padding))
    }

    fn spacing<T: num::NumCast>(self, s: T) -> Self where Self: Sized {
        self.add_attribute(Attribute::Spacing(num::cast(s).unwrap()))
    }

    fn align_center(self) -> Self where Self: Sized {
        self.add_attribute(Attribute::AlignmentX(AlignmentX::Center))
            .add_attribute(Attribute::AlignmentY(AlignmentY::Center))
    }

    fn align_center_x(self) -> Self where Self: Sized {
        self.add_attribute(Attribute::AlignmentX(AlignmentX::Center))
    }

    fn align_center_y(self) -> Self where Self: Sized {
        self.add_attribute(Attribute::AlignmentY(AlignmentY::Center))
    }

    fn align_top(self) -> Self where Self: Sized {
        self.add_attribute(Attribute::AlignmentY(AlignmentY::Top))
    }

    fn align_bottom(self) -> Self where Self: Sized {
        self.add_attribute(Attribute::AlignmentY(AlignmentY::Bottom))
    }

    fn align_left(self) -> Self where Self: Sized {
        self.add_attribute(Attribute::AlignmentX(AlignmentX::Left))
    }

    fn align_right(self) -> Self where Self: Sized {
        self.add_attribute(Attribute::AlignmentX(AlignmentX::Right))
    }

    fn disabled(self, is_disabled: bool) -> Self where Self: Sized {
        self.add_attribute(Attribute::Disabled(is_disabled))
    }

    fn add_attribute(mut self, attribute: Attribute) -> Self where Self: Sized {
        use Attribute::*;

        let mut cur = self.attributes_mut();
        match attribute {
            Width(la) => {
                cur.width = la
            },
            Height(la) => {
                cur.height = la
            },
            Padding(p) => {

                let padding = attributes::Padding {
                    left: p,
                    right: p,
                    top: p,
                    bottom: p
                };
                cur.padding = padding
            }
            PaddingXY(x,y) => {
                let padding = attributes::Padding {
                    left: x,
                    right: x,
                    top: y,
                    bottom: y,
                };
                cur.padding = padding
            },
            PaddingEach(padding) => {
                cur.padding = padding
            },

            WidthConstraint(constraint) => {
                cur.width_constraint = constraint;
            },

            HeightConstraint(constraint) => {
                cur.height_constraint = constraint;
            },

            Spacing(s) => {
                let spacing = attributes::Spacing {
                    x: s,
                    y: s
                };
                cur.spacing = spacing;
            },

            SpacingXY(x, y) => {
                let spacing = attributes::Spacing {
                    x,
                    y
                };
                cur.spacing = spacing;
            },

            Alignment(align) => {
                cur.align = align;
            },
            AlignmentX(x) => {
                cur.align.x = x;
            },
            AlignmentY(y) => {
                cur.align.y = y;
            },
            Attribute::Disabled(is_disabled) => {
                cur.disabled = is_disabled;
            }
        };

        self
    }
}
