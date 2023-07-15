use bevy::{math::Vec2, prelude::default, render::color::Color};

macro_rules! all_dyn {
    ($self:ident,
        ($($enumer:ident),*)
        => $capture:ident.$func:ident($name:ident)) => {
        match $self {
            $(
                Self::$enumer($capture) => $capture.$func($name),
            )*
        }
    };
}

#[derive(Debug, Clone, PartialEq)]
pub enum Brush {
    Color(Color),
    Gradient(Gradient),
}

impl Brusher for Brush {
    fn brush(&self, pos: Vec2) -> Color {
        all_dyn!(
            self,
            (Color, Gradient)
            => brush.brush(pos)
        )
    }
}

pub trait Brusher {
    fn brush(&self, pos: Vec2) -> Color;
}
impl Brusher for Color {
    fn brush(&self, _pos: Vec2) -> Color {
        *self
    }
}
impl Brusher for Gradient {
    fn brush(&self, pos: Vec2) -> Color {
        all_dyn!{
            self,
            (Linear) 
            => brush.brush(pos)
        }
    }
}

impl Into<Brush> for Color {
    fn into(self) -> Brush {
        Brush::Color(self)
    }
}

impl Into<Brush> for Gradient
{
    fn into(self) -> Brush {
        Brush::Gradient(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Gradient {
    Linear(LinearGradient),
}

impl Into<Gradient> for LinearGradient {
    fn into(self) -> Gradient {
        Gradient::Linear(self)
    }
}
#[derive(Default,Debug, Clone, PartialEq)]
pub struct LinearGradient {
    pub start: Vec2,
    pub end: Vec2,
    pub stops: Vec<GradientStop>,
}
impl LinearGradient {
    pub fn new_empty(start: Vec2, end: Vec2) -> Self {
        Self {
            start,
            end,
            ..default()
        }
    }

    pub fn add_stop(&mut self, offset: f32, color: Color) {
        self.stops.push(GradientStop::new(offset, color))
    }
}

impl Brusher for LinearGradient {
    fn brush(&self, pos: Vec2) -> Color {
        todo!("impl color_at in LinearGradient")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GradientStop {
    offset: f32,
    color: Color,
}

impl GradientStop {
    pub fn new(offset: f32, color: Color) -> Self {
        Self { offset, color }
    }
}
