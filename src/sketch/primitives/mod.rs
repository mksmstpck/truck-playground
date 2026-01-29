pub mod arc2d;
pub mod bspline2d;
pub mod circle2d;
pub mod line2d;
pub mod traits;

pub use arc2d::Arc2D;
pub use bspline2d::BSpline2D;
pub use circle2d::Circle2D;
pub use line2d::Line2D;
pub use traits::{BoundingBox2D, SketchCurve2D};

use truck_geometry::prelude::*;

/// Unified curve type for heterogeneous collections
#[derive(Clone, Debug)]
pub enum Curve2D {
    Line(Line2D),
    Arc(Arc2D),
    Circle(Circle2D),
    BSpline(BSpline2D),
}

impl Curve2D {
    /// Set start point (for gap healing) - only works for Line
    pub fn set_start(&mut self, p: Point2) {
        if let Curve2D::Line(line) = self {
            line.set_start(p);
        }
    }
}

impl SketchCurve2D for Curve2D {
    fn start(&self) -> Point2 {
        match self {
            Curve2D::Line(c) => c.start(),
            Curve2D::Arc(c) => c.start(),
            Curve2D::Circle(c) => c.start(),
            Curve2D::BSpline(c) => c.start(),
        }
    }

    fn end(&self) -> Point2 {
        match self {
            Curve2D::Line(c) => c.end(),
            Curve2D::Arc(c) => c.end(),
            Curve2D::Circle(c) => c.end(),
            Curve2D::BSpline(c) => c.end(),
        }
    }

    fn point_at(&self, t: f64) -> Point2 {
        match self {
            Curve2D::Line(c) => c.point_at(t),
            Curve2D::Arc(c) => c.point_at(t),
            Curve2D::Circle(c) => c.point_at(t),
            Curve2D::BSpline(c) => c.point_at(t),
        }
    }

    fn tangent_at(&self, t: f64) -> Vector2 {
        match self {
            Curve2D::Line(c) => c.tangent_at(t),
            Curve2D::Arc(c) => c.tangent_at(t),
            Curve2D::Circle(c) => c.tangent_at(t),
            Curve2D::BSpline(c) => c.tangent_at(t),
        }
    }

    fn length(&self) -> f64 {
        match self {
            Curve2D::Line(c) => c.length(),
            Curve2D::Arc(c) => c.length(),
            Curve2D::Circle(c) => c.length(),
            Curve2D::BSpline(c) => c.length(),
        }
    }

    fn reversed(&self) -> Self {
        match self {
            Curve2D::Line(c) => Curve2D::Line(c.reversed()),
            Curve2D::Arc(c) => Curve2D::Arc(c.reversed()),
            Curve2D::Circle(c) => Curve2D::Circle(c.reversed()),
            Curve2D::BSpline(c) => Curve2D::BSpline(c.reversed()),
        }
    }

    fn bounding_box(&self) -> BoundingBox2D {
        match self {
            Curve2D::Line(c) => c.bounding_box(),
            Curve2D::Arc(c) => c.bounding_box(),
            Curve2D::Circle(c) => c.bounding_box(),
            Curve2D::BSpline(c) => c.bounding_box(),
        }
    }
}

// Conversion From implementations
impl From<Line2D> for Curve2D {
    fn from(line: Line2D) -> Self {
        Curve2D::Line(line)
    }
}

impl From<Arc2D> for Curve2D {
    fn from(arc: Arc2D) -> Self {
        Curve2D::Arc(arc)
    }
}

impl From<Circle2D> for Curve2D {
    fn from(circle: Circle2D) -> Self {
        Curve2D::Circle(circle)
    }
}

impl From<BSpline2D> for Curve2D {
    fn from(spline: BSpline2D) -> Self {
        Curve2D::BSpline(spline)
    }
}
