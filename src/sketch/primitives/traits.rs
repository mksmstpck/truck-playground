use truck_geometry::prelude::*;

/// Common interface for all 2D sketch curves
pub trait SketchCurve2D: Clone + std::fmt::Debug {
    /// Starting point of the curve
    fn start(&self) -> Point2;

    /// Ending point of the curve
    fn end(&self) -> Point2;

    /// Point at parameter t ∈ [0, 1]
    fn point_at(&self, t: f64) -> Point2;

    /// Tangent vector at parameter t ∈ [0, 1] (not necessarily normalized)
    fn tangent_at(&self, t: f64) -> Vector2;

    /// Approximate arc length of the curve
    fn length(&self) -> f64;

    /// Return a reversed copy of the curve
    fn reversed(&self) -> Self
    where
        Self: Sized;

    /// Check if the curve is degenerate (zero length)
    fn is_degenerate(&self, tol: f64) -> bool {
        self.length() < tol
    }

    /// Check if curve is closed (start == end within tolerance)
    fn is_closed(&self, tol: f64) -> bool {
        (self.start() - self.end()).magnitude() < tol
    }

    /// Bounding box of the curve
    fn bounding_box(&self) -> BoundingBox2D;
}

#[derive(Clone, Debug)]
pub struct BoundingBox2D {
    pub min: Point2,
    pub max: Point2,
}

impl BoundingBox2D {
    pub fn new(min: Point2, max: Point2) -> Self {
        Self { min, max }
    }

    pub fn from_points(points: &[Point2]) -> Option<Self> {
        if points.is_empty() {
            return None;
        }

        let mut min = points[0];
        let mut max = points[0];

        for p in points.iter().skip(1) {
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
        }

        Some(Self { min, max })
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            min: Point2::new(self.min.x.min(other.min.x), self.min.y.min(other.min.y)),
            max: Point2::new(self.max.x.max(other.max.x), self.max.y.max(other.max.y)),
        }
    }

    #[allow(dead_code)]
    pub fn contains(&self, p: Point2) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }
}
