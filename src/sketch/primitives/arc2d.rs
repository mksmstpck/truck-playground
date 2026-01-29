use super::traits::{BoundingBox2D, SketchCurve2D};
use crate::sketch::constants::*;
use crate::sketch::error::*;
use std::f64::consts::{PI, TAU};
use truck_geometry::prelude::*;

/// A circular arc defined by center, radius, start angle, and sweep angle.
///
/// - `sweep_angle > 0` means counter-clockwise (CCW)
/// - `sweep_angle < 0` means clockwise (CW)
/// - `|sweep_angle|` must be in (0, 2π] for valid arcs
#[derive(Clone, Debug)]
pub struct Arc2D {
    center: Point2,
    radius: f64,
    start_angle: f64,
    sweep_angle: f64,
}

impl Arc2D {
    /// Create arc from center, radius, start angle, and sweep angle
    pub fn new(
        center: Point2,
        radius: f64,
        start_angle: f64,
        sweep_angle: f64,
    ) -> SketchResult<Self> {
        if radius <= DEGENERATE_TOLERANCE {
            return Err(SketchError::InvalidArcRadius(radius));
        }
        if sweep_angle.abs() < ANGLE_TOLERANCE {
            return Err(SketchError::ZeroSweepAngle);
        }

        // Clamp sweep to [-2π, 2π]
        let clamped_sweep = sweep_angle.clamp(-TAU, TAU);

        Ok(Self {
            center,
            radius,
            start_angle: normalize_angle(start_angle),
            sweep_angle: clamped_sweep,
        })
    }

    /// Create arc from start point, end point, and center
    pub fn from_start_end_center(
        start: Point2,
        end: Point2,
        center: Point2,
        ccw: bool,
    ) -> SketchResult<Self> {
        let r1 = (start - center).magnitude();
        let r2 = (end - center).magnitude();

        // Check radii match within tolerance
        if (r1 - r2).abs() > LENGTH_TOLERANCE * r1.max(r2).max(1.0) {
            return Err(SketchError::ArcRadiusMismatch { r1, r2 });
        }

        let radius = (r1 + r2) / 2.0;
        if radius <= DEGENERATE_TOLERANCE {
            return Err(SketchError::InvalidArcRadius(radius));
        }

        let start_angle = (start.y - center.y).atan2(start.x - center.x);
        let end_angle = (end.y - center.y).atan2(end.x - center.x);

        let sweep_angle = compute_sweep_angle(start_angle, end_angle, ccw);

        Self::new(center, radius, start_angle, sweep_angle)
    }

    /// Create arc from three points (start, point on arc, end)
    pub fn from_three_points(start: Point2, mid: Point2, end: Point2) -> SketchResult<Self> {
        let center = circumcenter(start, mid, end)?;
        let radius = (start - center).magnitude();

        let start_angle = (start.y - center.y).atan2(start.x - center.x);
        let mid_angle = (mid.y - center.y).atan2(mid.x - center.x);
        let end_angle = (end.y - center.y).atan2(end.x - center.x);

        let sweep_angle = compute_sweep_through_mid(start_angle, mid_angle, end_angle);

        Self::new(center, radius, start_angle, sweep_angle)
    }

    // Getters
    pub fn center(&self) -> Point2 {
        self.center
    }
    pub fn radius(&self) -> f64 {
        self.radius
    }
    #[allow(dead_code)]
    pub fn start_angle(&self) -> f64 {
        self.start_angle
    }
    pub fn sweep_angle(&self) -> f64 {
        self.sweep_angle
    }
    #[allow(dead_code)]
    pub fn end_angle(&self) -> f64 {
        self.start_angle + self.sweep_angle
    }
    pub fn is_ccw(&self) -> bool {
        self.sweep_angle > 0.0
    }

    /// Angle at parameter t ∈ [0, 1]
    fn angle_at(&self, t: f64) -> f64 {
        self.start_angle + t * self.sweep_angle
    }
}

impl SketchCurve2D for Arc2D {
    fn start(&self) -> Point2 {
        let angle = self.start_angle;
        Point2::new(
            self.center.x + self.radius * angle.cos(),
            self.center.y + self.radius * angle.sin(),
        )
    }

    fn end(&self) -> Point2 {
        let angle = self.start_angle + self.sweep_angle;
        Point2::new(
            self.center.x + self.radius * angle.cos(),
            self.center.y + self.radius * angle.sin(),
        )
    }

    fn point_at(&self, t: f64) -> Point2 {
        let angle = self.angle_at(t);
        Point2::new(
            self.center.x + self.radius * angle.cos(),
            self.center.y + self.radius * angle.sin(),
        )
    }

    fn tangent_at(&self, t: f64) -> Vector2 {
        let angle = self.angle_at(t);
        let sign = if self.sweep_angle >= 0.0 { 1.0 } else { -1.0 };
        Vector2::new(-angle.sin() * sign, angle.cos() * sign)
    }

    fn length(&self) -> f64 {
        self.radius * self.sweep_angle.abs()
    }

    fn reversed(&self) -> Self {
        Self {
            center: self.center,
            radius: self.radius,
            start_angle: normalize_angle(self.start_angle + self.sweep_angle),
            sweep_angle: -self.sweep_angle,
        }
    }

    fn bounding_box(&self) -> BoundingBox2D {
        // Start with endpoints
        let mut points = vec![self.start(), self.end()];

        // Check if arc crosses cardinal directions (0, π/2, π, 3π/2)
        let cardinals = [0.0, PI / 2.0, PI, 3.0 * PI / 2.0];

        let (angle_min, angle_max) = if self.sweep_angle >= 0.0 {
            (self.start_angle, self.start_angle + self.sweep_angle)
        } else {
            (self.start_angle + self.sweep_angle, self.start_angle)
        };

        for &cardinal in &cardinals {
            // Check both cardinal and cardinal + 2π
            for offset in [0.0, TAU, -TAU] {
                let c = cardinal + offset;
                if c > angle_min && c < angle_max {
                    points.push(Point2::new(
                        self.center.x + self.radius * cardinal.cos(),
                        self.center.y + self.radius * cardinal.sin(),
                    ));
                }
            }
        }

        BoundingBox2D::from_points(&points).unwrap()
    }
}

// Helper functions

fn normalize_angle(angle: f64) -> f64 {
    let a = angle % TAU;
    if a < 0.0 {
        a + TAU
    } else {
        a
    }
}

fn compute_sweep_angle(start: f64, end: f64, ccw: bool) -> f64 {
    let mut sweep = end - start;

    if ccw {
        while sweep <= 0.0 {
            sweep += TAU;
        }
    } else {
        while sweep >= 0.0 {
            sweep -= TAU;
        }
    }

    sweep
}

fn compute_sweep_through_mid(start: f64, mid: f64, end: f64) -> f64 {
    let s = normalize_angle(start);
    let m = normalize_angle(mid);
    let e = normalize_angle(end);

    // CCW distances
    let s_to_m_ccw = if m >= s { m - s } else { m - s + TAU };
    let s_to_e_ccw = if e >= s { e - s } else { e - s + TAU };

    // Check if mid is between start and end going CCW
    if s_to_m_ccw < s_to_e_ccw {
        s_to_e_ccw // CCW direction
    } else {
        s_to_e_ccw - TAU // CW direction
    }
}

fn circumcenter(p1: Point2, p2: Point2, p3: Point2) -> SketchResult<Point2> {
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
