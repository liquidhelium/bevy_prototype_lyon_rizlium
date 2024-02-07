use bevy::{math::Vec2, prelude::default, reflect::Reflect, render::color::Color};
use lyon_algorithms::geom::euclid::approxeq::ApproxEq;

use crate::render::GradientMaterialUniform;

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

#[derive(Debug, Clone, PartialEq, Reflect)]
#[reflect(PartialEq)]
pub enum Brush {
    Color(Color),
    Gradient(Gradient),
}

impl Brush {
    pub fn clone_as_uniform(&self) -> GradientMaterialUniform {
        match self {
            Self::Color(color) => GradientMaterialUniform {
                start: color.clone(),
                end: color.clone(),
                ..default()
            },
            Self::Gradient(Gradient::Linear(ref linear)) => {
                let start = linear
                    .stops
                    .first()
                    .cloned()
                    .map(|o| o.color)
                    .unwrap_or(Color::BLACK);
                let end = linear
                    .stops
                    .last()
                    .cloned()
                    .map(|o| o.color)
                    .unwrap_or(Color::BLACK);
                let start_pos = linear.start;
                let end_pos = linear.end;
                GradientMaterialUniform {
                    start,
                    end,
                    start_pos,
                    end_pos,
                }
            }
        }
    }
}

impl Default for Brush {
    fn default() -> Self {
        Self::Color(default())
    }
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
        all_dyn! {
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

impl Into<Brush> for Gradient {
    fn into(self) -> Brush {
        Brush::Gradient(self)
    }
}

#[derive(Debug, Clone, PartialEq, Reflect)]
#[reflect(PartialEq)]
pub enum Gradient {
    Linear(LinearGradient),
}

impl Into<Gradient> for LinearGradient {
    fn into(self) -> Gradient {
        Gradient::Linear(self)
    }
}
#[derive(Default, Debug, Clone, PartialEq, Reflect)]
#[reflect(PartialEq)]
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

    fn progress(&self, pos: Vec2) -> f32 {
        let vec = self.end - self.start;
        let length = vec.length();
        let posing_vec = pos - self.start;
        if !length.approx_eq(&0.) {
            let product = vec.dot(posing_vec);
            return product / (length * length);
        } else {
            0.
        }
    }
}
#[derive(Clone, Copy, PartialEq, PartialOrd)]
struct NonNan(f32);
impl NonNan {
    fn new_checked(val: f32) -> Option<Self> {
        if val.is_nan() {
            None
        } else {
            Some(Self(val))
        }
    }
}
impl Eq for NonNan {}
impl Ord for NonNan {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl Brusher for LinearGradient {
    fn brush(&self, pos: Vec2) -> Color {
        if self.stops.len() <= 1 {
            return self.stops.get(0).map(|i| i.color).unwrap_or(Color::NONE);
        }
        if self.stops.len() == 2 {
            if self.stops[0] == self.stops[1] {
                return self.stops[0].color;
            }
        }
        let progress = NonNan::new_checked(self.progress(pos).clamp(0., 1.)).unwrap();
        let (Ok(index) | Err(index)) = self
            .stops
            .binary_search_by_key(&progress, |item| NonNan::new_checked(item.offset).unwrap());
        let index = index.clamp(0, self.stops.len() - 2);
        let former = self.stops[index];
        let latter = self.stops[index + 1];
        let mut t = (progress.0 - former.offset) / (latter.offset - former.offset);
        if t.is_nan() {
            t = 0.;
        }
        let color1 = former.color.as_linear_rgba_f32();
        let color2 = latter.color.as_linear_rgba_f32();
        let lerp: Vec<_> = color1
            .into_iter()
            .zip(color2)
            .map(|(a, b)| t * (b - a) + a)
            .collect();
        Color::RgbaLinear {
            red: lerp[0],
            green: lerp[1],
            blue: lerp[2],
            alpha: lerp[3],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(PartialEq)]
pub struct GradientStop {
    offset: f32,
    color: Color,
}

impl GradientStop {
    pub fn new(offset: f32, color: Color) -> Self {
        Self { offset, color }
    }
}
