use bevy::{
    math::{Mat3, Vec2},
    prelude::default,
    render::color::Color,
};
use lyon_algorithms::geom::euclid::{approxeq::ApproxEq, default};

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

#[derive(Debug, Clone, PartialEq)]
pub enum Gradient {
    Linear(LinearGradient),
}

impl Into<Gradient> for LinearGradient {
    fn into(self) -> Gradient {
        Gradient::Linear(self)
    }
}
#[derive(Default, Debug, Clone, PartialEq)]
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
        if !length.approx_eq(&0.) {
            let gradient_vec = vec.normalize();
            let sin = gradient_vec.y;
            let cos = gradient_vec.x;
            let mut mat =
                Mat3::from_cols_array_2d(&[[cos, -sin, 0.], [sin, cos, 0.], [0., 0., 1.]]);
            mat *= Mat3::from_translation(-self.start);
            mat *= 1. / length;
            mat.transform_vector2(pos).x
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
            return self.stops.get(0).map(|i| i.color).unwrap_or(Color::BLACK);
        }
        let progress = NonNan::new_checked(self.progress(pos).clamp(0., 1.)).unwrap();
        let (Ok(index) | Err(index)) = self
            .stops
            .binary_search_by_key(&progress, |item| NonNan::new_checked(item.offset).unwrap());
        let index = index.clamp(0, self.stops.len() - 2);
        let former = self.stops[index];
        let latter = self.stops[index];
        let t = (progress.0 - former.offset) / (latter.offset - former.offset);
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
