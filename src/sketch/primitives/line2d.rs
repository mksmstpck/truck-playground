use super::traits::{BoundingBox2D, SketchCurve2D};
use crate::sketch::constants::*;
use crate::sketch::error::*;
use truck_geometry::prelude::*;

#[derive(Clone, Debug)]
pub struct Line2D {
    start: Point2,
    end: Point2,
}

impl Line2D {
    /// Create a new line segment
    pub fn new(start: Point2, end: Point2) -> SketchResult<Self> {
        let line = Self { start, end };
        if line.is_degenerate(DEGENERATE_TOLERANCE) {
            return Err(SketchError::DegenerateCurve);
        }
        Ok(line)
    }

    /// Create without validation (internal use)
    pub(crate) fn new_unchecked(start: Point2, end: Point2) -> Self {
        Self { start, end }
    }

    /// Direction vector (normalized)
    #[allow(dead_code)]
    pub fn direction(&self) -> Vector2 {
        (self.end - self.start).normalize()
    }

    /// Midpoint of the line
    #[allow(dead_code)]
    pub fn midpoint(&self) -> Point2 {
        Point2::new(
            (self.start.x + self.end.x) / 2.0,
            (self.start.y + self.end.y) / 2.0,
        )
    }

    /// Set start point (for gap healing)
    pub fn set_start(&mut self, p: Point2) {
        self.start = p;
    }

    /// Set end point (for gap healing)
    #[allow(dead_code)]
    pub fn set_end(&mut self, p: Point2) {
        self.end = p;
    }
}

impl SketchCurve2D for Line2D {
    fn start(&self) -> Point2 {
        self.start
    }

    fn end(&self) -> Point2 {
        self.end
    }

    fn point_at(&self, t: f64) -> Point2 {
        Point2::new(
            self.start.x + t * (self.end.x - self.start.x),
            self.start.y + t * (self.end.y - self.start.y),
        )
    }

    fn tangent_at(&self, _t: f64) -> Vector2 {
        self.end - self.start
    }

    fn length(&self) -> f64 {
        (self.end - self.start).magnitude()
    }

    fn reversed(&self) -> Self {
        Self {
            start: self.end,
            end: self.start,
        }
    }

    fn bounding_box(&self) -> BoundingBox2D {
        BoundingBox2D::new(
            Point2::new(self.start.x.min(self.end.x), self.start.y.min(self.end.y)),
            Point2::new(self.start.x.max(self.end.x), self.start.y.max(self.end.y)),
        )
    }
}
