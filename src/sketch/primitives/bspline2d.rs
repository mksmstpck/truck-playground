use super::traits::{BoundingBox2D, SketchCurve2D};
use crate::sketch::error::*;
use std::ops::Bound;
use truck_geometry::prelude::*;

#[derive(Clone, Debug)]
pub struct BSpline2D {
    curve: BSplineCurve<Point2>,
}

impl BSpline2D {
    /// Create from control points with automatic uniform knot vector
    pub fn from_control_points(points: Vec<Point2>, degree: usize) -> SketchResult<Self> {
        let n = points.len();
        let min_points = degree + 1;

        if n < min_points {
            return Err(SketchError::InsufficientControlPoints {
                min: min_points,
                degree,
                got: n,
            });
        }

        let knots = KnotVec::uniform_knot(n, degree);
        let curve = BSplineCurve::new(knots, points);

        Ok(Self { curve })
    }

    /// Create from existing truck B-spline curve
    #[allow(dead_code)]
    pub fn from_truck_curve(curve: BSplineCurve<Point2>) -> Self {
        Self { curve }
    }

    /// Create interpolating spline through points
    #[allow(dead_code)]
    pub fn interpolate(points: &[Point2], degree: usize) -> SketchResult<Self> {
        if points.len() < 2 {
            return Err(SketchError::InsufficientControlPoints {
                min: 2,
                degree,
                got: points.len(),
            });
        }

        // For simplicity, use control points as-is for low point counts
        // A full implementation would solve the linear system
        Self::from_control_points(points.to_vec(), degree.min(points.len() - 1))
    }

    /// Get the underlying truck curve
    pub fn inner(&self) -> &BSplineCurve<Point2> {
        &self.curve
    }

    /// Get degree of the spline
    #[allow(dead_code)]
    pub fn degree(&self) -> usize {
        self.curve.degree()
    }

    /// Get control points
    pub fn control_points(&self) -> &[Point2] {
        self.curve.control_points()
    }

    fn param_range(&self) -> (f64, f64) {
        let (b0, b1) = self.curve.parameter_range();
        (bound_value(b0), bound_value(b1))
    }
}

impl SketchCurve2D for BSpline2D {
    fn start(&self) -> Point2 {
        let (t0, _) = self.param_range();
        self.curve.subs(t0)
    }

    fn end(&self) -> Point2 {
        let (_, t1) = self.param_range();
        self.curve.subs(t1)
    }

    fn point_at(&self, t: f64) -> Point2 {
        let (t0, t1) = self.param_range();
        let param = t0 + t * (t1 - t0);
        self.curve.subs(param)
    }

    fn tangent_at(&self, t: f64) -> Vector2 {
        let (t0, t1) = self.param_range();
        let param = t0 + t * (t1 - t0);
        self.curve.der(param)
    }

    fn length(&self) -> f64 {
        // Approximate using sampling
        const SAMPLES: usize = 100;
        let mut len = 0.0;
        let mut prev = self.start();

        for i in 1..=SAMPLES {
            let t = i as f64 / SAMPLES as f64;
            let curr = self.point_at(t);
            len += (curr - prev).magnitude();
            prev = curr;
        }

        len
    }

    fn reversed(&self) -> Self {
        let mut reversed = self.curve.clone();
        reversed.invert();
        Self { curve: reversed }
    }

    fn bounding_box(&self) -> BoundingBox2D {
        // Use control points as conservative estimate
        BoundingBox2D::from_points(self.curve.control_points()).unwrap()
    }
}

fn bound_value(b: Bound<f64>) -> f64 {
    match b {
        Bound::Included(t) | Bound::Excluded(t) => t,
        Bound::Unbounded => panic!("Unbounded spline parameter"),
    }
}
