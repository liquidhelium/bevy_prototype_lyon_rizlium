//! Types for defining shape color and options.

use bevy::{ecs::component::Component, render::color::Color};
use lyon_tessellation::{FillOptions, StrokeOptions};

use crate::brush::Brush;

/// Defines the fill options for the lyon tessellator and color of the generated
/// vertices.
#[allow(missing_docs)]
#[derive(Component, Debug, Clone, PartialEq)]
pub struct Fill {
    pub options: FillOptions,
    pub brush: Brush,
}

impl Fill {
    /// Convenience constructor requiring only the `Color`.
    #[must_use]
    pub fn brush(brush: impl Into<Brush>) -> Self {
        Self {
            options: FillOptions::default(),
            brush: brush.into(),
        }
    }
}

/// Defines the stroke options for the lyon tessellator and color of the
/// generated vertices.
#[allow(missing_docs)]
#[derive(Component, Debug, Clone, PartialEq)]
pub struct Stroke {
    pub options: StrokeOptions,
    pub brush: Brush,
}

impl Stroke {
    /// Constructor that requires a `Color` and a line width.
    #[must_use]
    pub fn new(brush: impl Into<Brush>, line_width: f32) -> Self {
        Self {
            options: StrokeOptions::default().with_line_width(line_width),
            brush: brush.into(),
        }
    }

    /// Convenience constructor requiring only the `Color`.
    #[must_use]
    pub fn color(color: Color) -> Self {
        Self {
            options: StrokeOptions::default(),
            brush: color.into(),
        }
    }
}
