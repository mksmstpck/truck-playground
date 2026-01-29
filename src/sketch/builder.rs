use crate::sketch::constants::*;
use crate::sketch::error::*;
use crate::sketch::loop2d::Loop2D;
use crate::sketch::primitives::{Arc2D, BSpline2D, Curve2D, Line2D};
use truck_geometry::prelude::*;

/// Fluent builder for creating sketch loops
pub struct SketchBuilder {
    curves: Vec<Curve2D>,
    current_pos: Option<Point2>,
    start_pos: Option<Point2>,
}

impl SketchBuilder {
    /// Create a new empty builder
    pub fn new() -> Self {
        Self {
            curves: Vec::new(),
            current_pos: None,
            start_pos: None,
        }
    }

    /// Start at a point (required before drawing)
    pub fn move_to(mut self, pt: Point2) -> Self {
        self.current_pos = Some(pt);
        if self.start_pos.is_none() {
            self.start_pos = Some(pt);
        }
        self
    }

    /// Draw a line to a point
    pub fn line_to(mut self, pt: Point2) -> SketchResult<Self> {
        let start = self.current_pos.ok_or(SketchError::NoStartingPoint)?;

        let line = Line2D::new(start, pt)?;
        self.curves.push(Curve2D::Line(line));
        self.current_pos = Some(pt);

        Ok(self)
    }

    /// Draw a horizontal line by dx
    pub fn horizontal(self, dx: f64) -> SketchResult<Self> {
        let current = self.current_pos.ok_or(SketchError::NoStartingPoint)?;
        self.line_to(Point2::new(current.x + dx, current.y))
    }

    /// Draw a vertical line by dy
    pub fn vertical(self, dy: f64) -> SketchResult<Self> {
        let current = self.current_pos.ok_or(SketchError::NoStartingPoint)?;
        self.line_to(Point2::new(current.x, current.y + dy))
    }

    /// Draw a line by relative offset
    #[allow(dead_code)]
    pub fn line_by(self, dx: f64, dy: f64) -> SketchResult<Self> {
        let current = self.current_pos.ok_or(SketchError::NoStartingPoint)?;
        self.line_to(Point2::new(current.x + dx, current.y + dy))
    }

    /// Draw an arc to a point with given center
    pub fn arc_to(mut self, end: Point2, center: Point2, ccw: bool) -> SketchResult<Self> {
        let start = self.current_pos.ok_or(SketchError::NoStartingPoint)?;

        let arc = Arc2D::from_start_end_center(start, end, center, ccw)?;
        self.curves.push(Curve2D::Arc(arc));
        self.current_pos = Some(end);

        Ok(self)
    }

    /// Draw an arc through three points (start is current position)
    #[allow(dead_code)]
    pub fn arc_through(mut self, mid: Point2, end: Point2) -> SketchResult<Self> {
        let start = self.current_pos.ok_or(SketchError::NoStartingPoint)?;

        let arc = Arc2D::from_three_points(start, mid, end)?;
        self.curves.push(Curve2D::Arc(arc));
        self.current_pos = Some(end);

        Ok(self)
    }

    /// Draw an arc with radius, sweep angle, and direction
    #[allow(dead_code)]
    pub fn arc_by_angle(mut self, radius: f64, sweep_angle: f64, ccw: bool) -> SketchResult<Self> {
        let start = self.current_pos.ok_or(SketchError::NoStartingPoint)?;

        // Get tangent direction from previous curve or default to +X
        let tangent = if let Some(last) = self.curves.last() {
            use crate::sketch::primitives::SketchCurve2D;
            last.tangent_at(1.0).normalize()
        } else {
            Vector2::new(1.0, 0.0)
        };

        // Center is perpendicular to tangent at distance radius
        let perp = if ccw {
            Vector2::new(-tangent.y, tangent.x)
        } else {
            Vector2::new(tangent.y, -tangent.x)
        };
        let center = start + perp * radius;

        let start_angle = (start.y - center.y).atan2(start.x - center.x);
        let actual_sweep = if ccw {
            sweep_angle.abs()
        } else {
            -sweep_angle.abs()
        };

        let arc = Arc2D::new(center, radius, start_angle, actual_sweep)?;
        let end = {
            use crate::sketch::primitives::SketchCurve2D;
            arc.end()
        };

        self.curves.push(Curve2D::Arc(arc));
        self.current_pos = Some(end);

        Ok(self)
    }

    /// Draw a quadratic Bezier curve
    #[allow(dead_code)]
    pub fn quadratic_to(mut self, control: Point2, end: Point2) -> SketchResult<Self> {
        let start = self.current_pos.ok_or(SketchError::NoStartingPoint)?;

        // Convert quadratic to cubic for uniform representation
        let cp1 = start + (control - start) * (2.0 / 3.0);
        let cp2 = end + (control - end) * (2.0 / 3.0);

        let spline = BSpline2D::from_control_points(vec![start, cp1, cp2, end], 3)?;
        self.curves.push(Curve2D::BSpline(spline));
        self.current_pos = Some(end);

        Ok(self)
    }

    /// Draw a cubic Bezier curve
    #[allow(dead_code)]
    pub fn cubic_to(mut self, cp1: Point2, cp2: Point2, end: Point2) -> SketchResult<Self> {
        let start = self.current_pos.ok_or(SketchError::NoStartingPoint)?;

        let spline = BSpline2D::from_control_points(vec![start, cp1, cp2, end], 3)?;
        self.curves.push(Curve2D::BSpline(spline));
        self.current_pos = Some(end);

        Ok(self)
    }

    /// Draw a spline through points (interpolating)
    #[allow(dead_code)]
    pub fn spline_through(mut self, points: &[Point2]) -> SketchResult<Self> {
        let start = self.current_pos.ok_or(SketchError::NoStartingPoint)?;

        let mut all_points = vec![start];
        all_points.extend_from_slice(points);

        let spline = BSpline2D::interpolate(&all_points, 3)?;
        let end = *points.last().ok_or(SketchError::DegenerateCurve)?;

        self.curves.push(Curve2D::BSpline(spline));
        self.current_pos = Some(end);

        Ok(self)
    }

    /// Close the loop with a line back to start
    pub fn close(mut self) -> SketchResult<Loop2D> {
        if self.curves.is_empty() {
            return Err(SketchError::CannotCloseEmpty);
        }

        let start = self.start_pos.ok_or(SketchError::NoStartingPoint)?;
        let current = self.current_pos.ok_or(SketchError::NoStartingPoint)?;

        // Add closing line if not already at start
        let gap = (current - start).magnitude();
        if gap > POINT_TOLERANCE {
            let line = Line2D::new_unchecked(current, start);
            self.curves.push(Curve2D::Line(line));
        }

        Loop2D::new(self.curves)
    }

    /// Close with an arc
    pub fn close_with_arc(mut self, center: Point2, ccw: bool) -> SketchResult<Loop2D> {
        if self.curves.is_empty() {
            return Err(SketchError::CannotCloseEmpty);
        }

        let start_pos = self.start_pos.ok_or(SketchError::NoStartingPoint)?;
        let current = self.current_pos.ok_or(SketchError::NoStartingPoint)?;

        let arc = Arc2D::from_start_end_center(current, start_pos, center, ccw)?;
        self.curves.push(Curve2D::Arc(arc));

        Loop2D::new(self.curves)
    }

    /// Build without closing (returns curves)
    #[allow(dead_code)]
    pub fn build_open(self) -> Vec<Curve2D> {
        self.curves
    }

    /// Get current position
    #[allow(dead_code)]
    pub fn current_position(&self) -> Option<Point2> {
        self.current_pos
    }

    /// Get start position
    #[allow(dead_code)]
    pub fn start_position(&self) -> Option<Point2> {
        self.start_pos
    }

    /// Get number of curves so far
    #[allow(dead_code)]
    pub fn curve_count(&self) -> usize {
        self.curves.len()
    }
}

impl Default for SketchBuilder {
    fn default() -> Self {
        Self::new()
    }
}
