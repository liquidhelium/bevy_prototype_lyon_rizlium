use bevy::{color::{Color, ColorToComponents, LinearRgba}, math::Vec2, prelude::default, reflect::Reflect};
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
    #[must_use] pub fn clone_as_uniform(&self) -> GradientMaterialUniform {
        match self {
            Self::Color(color) => GradientMaterialUniform {
                start: (*color).into(),
                end: (*color).into(),
                ..default()
            },
            Self::Gradient(Gradient::Linear(ref linear)) => {
                let start = linear
                    .stops
                    .first()
                    .copied()
                    .map_or(Color::BLACK, |o| o.color);
                let end = linear
                    .stops
                    .last()
                    .copied()
                    .map_or(Color::BLACK, |o| o.color);
                let start_pos = linear.start;
                let end_pos = linear.end;
                GradientMaterialUniform {
                    start: start.into(),
                    end: end.into(),
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

impl From<Color> for Brush {
    fn from(val: Color) -> Self {
        Brush::Color(val)
    }
}

impl From<Gradient> for Brush {
    fn from(val: Gradient) -> Self {
        Brush::Gradient(val)
    }
}

#[derive(Debug, Clone, PartialEq, Reflect)]
#[reflect(PartialEq)]
pub enum Gradient {
    Linear(LinearGradient),
}

impl From<LinearGradient> for Gradient {
    fn from(val: LinearGradient) -> Self {
        Gradient::Linear(val)
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
    #[must_use] pub fn new_empty(start: Vec2, end: Vec2) -> Self {
        Self {
            start,
            end,
            ..default()
        }
    }

    pub fn add_stop(&mut self, offset: f32, color: Color) {
        self.stops.push(GradientStop::new(offset, color));
    }

    fn progress(&self, pos: Vec2) -> f32 {
        let vec = self.end - self.start;
        let length = vec.length();
        let posing_vec = pos - self.start;
        if !length.approx_eq(&0.) {
            let product = vec.dot(posing_vec);
            product / (length * length)
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
            return self.stops.first().map_or(Color::NONE, |i| i.color);
        }
        if self.stops.len() == 2 && self.stops[0] == self.stops[1] {
            return self.stops[0].color;
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
        let color1 = LinearRgba::from(former.color).to_f32_array();
        let color2 = LinearRgba::from(latter.color).to_f32_array();
        let lerp: Vec<_> = color1
            .into_iter()
            .zip(color2)
            .map(|(a, b)| t * (b - a) + a)
            .collect();
        LinearRgba {
            red: lerp[0],
            green: lerp[1],
            blue: lerp[2],
            alpha: lerp[3],
        }.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
#[reflect(PartialEq)]
pub struct GradientStop {
    offset: f32,
    color: Color,
}

impl GradientStop {
    #[must_use] pub fn new(offset: f32, color: Color) -> Self {
        Self { offset, color }
    }
}