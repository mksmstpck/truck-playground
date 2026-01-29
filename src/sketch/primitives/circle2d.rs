use super::arc2d::Arc2D;
use super::traits::{BoundingBox2D, SketchCurve2D};
use crate::sketch::constants::*;
use crate::sketch::error::*;
use std::f64::consts::{PI, TAU};
use truck_geometry::prelude::*;

/// A full circle, which is a special closed curve.
///
/// Unlike Arc2D, a Circle2D always represents a complete 360° curve.
/// It has a seam point where start() == end().
#[derive(Clone, Debug)]
pub struct Circle2D {
    center: Point2,
    radius: f64,
    /// Angle of the seam point (where start/end meet)
    seam_angle: f64,
    /// true = CCW (default), false = CW
    ccw: bool,
}

impl Circle2D {
    /// Create a new circle
    pub fn new(center: Point2, radius: f64) -> SketchResult<Self> {
        Self::with_seam(center, radius, 0.0, true)
    }

    /// Create a circle with specified seam angle and direction
    pub fn with_seam(center: Point2, radius: f64, seam_angle: f64, ccw: bool) -> SketchResult<Self> {
        if radius <= DEGENERATE_TOLERANCE {
            return Err(SketchError::InvalidCircleRadius(radius));
        }

        Ok(Self {
            center,
            radius,
            seam_angle,
            ccw,
        })
    }

    /// Create circle from three points on the circumference
    pub fn from_three_points(p1: Point2, p2: Point2, p3: Point2) -> SketchResult<Self> {
        let center = circumcenter_from_three(p1, p2, p3)?;
        let radius = (p1 - center).magnitude();

        Self::new(center, radius)
    }

    /// Create circle from center and a point on circumference
    #[allow(dead_code)]
    pub fn from_center_point(center: Point2, point_on_circle: Point2) -> SketchResult<Self> {
        let radius = (point_on_circle - center).magnitude();
        let seam_angle = (point_on_circle.y - center.y).atan2(point_on_circle.x - center.x);

        Self::with_seam(center, radius, seam_angle, true)
    }

    /// Create circle from diameter endpoints
    #[allow(dead_code)]
    pub fn from_diameter(p1: Point2, p2: Point2) -> SketchResult<Self> {
        let center = Point2::new((p1.x + p2.x) / 2.0, (p1.y + p2.y) / 2.0);
        let radius = (p1 - center).magnitude();

        if radius <= DEGENERATE_TOLERANCE {
            return Err(SketchError::InvalidCircleRadius(radius));
        }

        Self::new(center, radius)
    }

    // Getters
    pub fn center(&self) -> Point2 {
        self.center
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
    #[allow(dead_code)]
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
    #[allow(dead_code)]
    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    #[allow(dead_code)]
    pub fn circumference(&self) -> f64 {
        TAU * self.radius
    }
    pub fn is_ccw(&self) -> bool {
        self.ccw
    }

    /// Convert to an Arc2D (full 360° arc)
    pub fn to_arc(&self) -> Arc2D {
        let sweep = if self.ccw { TAU } else { -TAU };
        // Safe because we validated radius in constructor
        Arc2D::new(self.center, self.radius, self.seam_angle, sweep).unwrap()
    }

    /// Check if a point is inside the circle
    #[allow(dead_code)]
    pub fn contains_point(&self, p: Point2) -> bool {
        (p - self.center).magnitude() < self.radius
    }

    /// Get point at angle (in radians)
    pub fn point_at_angle(&self, angle: f64) -> Point2 {
        Point2::new(
            self.center.x + self.radius * angle.cos(),
            self.center.y + self.radius * angle.sin(),
        )
    }
}

impl SketchCurve2D for Circle2D {
    fn start(&self) -> Point2 {
        self.point_at_angle(self.seam_angle)
    }

    fn end(&self) -> Point2 {
        self.start() // Circle is closed
    }

    fn point_at(&self, t: f64) -> Point2 {
        let sweep = if self.ccw { TAU } else { -TAU };
        let angle = self.seam_angle + t * sweep;
        self.point_at_angle(angle)
    }

    fn tangent_at(&self, t: f64) -> Vector2 {
        let sweep = if self.ccw { TAU } else { -TAU };
        let angle = self.seam_angle + t * sweep;
        let sign = if self.ccw { 1.0 } else { -1.0 };
        Vector2::new(-angle.sin() * sign, angle.cos() * sign)
    }

    fn length(&self) -> f64 {
        TAU * self.radius
    }

    fn reversed(&self) -> Self {
        Self {
            center: self.center,
            radius: self.radius,
            seam_angle: self.seam_angle,
            ccw: !self.ccw,
        }
    }

    fn is_closed(&self, _tol: f64) -> bool {
        true // Always closed by definition
    }

    fn bounding_box(&self) -> BoundingBox2D {
        BoundingBox2D::new(
            Point2::new(self.center.x - self.radius, self.center.y - self.radius),
            Point2::new(self.center.x + self.radius, self.center.y + self.radius),
        )
    }
}

// Helper
fn circumcenter_from_three(p1: Point2, p2: Point2, p3: Point2) -> SketchResult<Point2> {
    let d = 2.0 * (p1.x * (p2.y - p3.y) + p2.x * (p3.y - p1.y) + p3.x * (p1.y - p2.y));

    if d.abs() < DEGENERATE_TOLERANCE {
        return Err(SketchError::CollinearPoints);
    }

    let p1_sq = p1.x * p1.x + p1.y * p1.y;
    let p2_sq = p2.x * p2.x + p2.y * p2.y;
    let p3_sq = p3.x * p3.x + p3.y * p3.y;

    let ux = (p1_sq * (p2.y - p3.y) + p2_sq * (p3.y - p1.y) + p3_sq * (p1.y - p2.y)) / d;
    let uy = (p1_sq * (p3.x - p2.x) + p2_sq * (p1.x - p3.x) + p3_sq * (p2.x - p1.x)) / d;

    Ok(Point2::new(ux, uy))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_is_closed() {
        let circle = Circle2D::new(Point2::origin(), 10.0).unwrap();
        assert!(circle.is_closed(1e-10));
    }

    #[test]
    fn test_invalid_radius() {
        assert!(Circle2D::new(Point2::origin(), 0.0).is_err());
        assert!(Circle2D::new(Point2::origin(), -5.0).is_err());
    }

    #[test]
    fn test_circle_length() {
        let circle = Circle2D::new(Point2::origin(), 1.0).unwrap();
        assert!((circle.length() - TAU).abs() < 1e-10);
    }

    #[test]
    fn test_circle_points() {
        let circle = Circle2D::new(Point2::origin(), 10.0).unwrap();
        let start = circle.start();
        let end = circle.end();
        assert!((start - end).magnitude() < 1e-10);
    }
}
