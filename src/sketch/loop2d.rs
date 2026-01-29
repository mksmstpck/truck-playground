use truck_modeling::InnerSpace;

use crate::sketch::constants::*;
use crate::sketch::error::*;
use crate::sketch::primitives::{BoundingBox2D, Curve2D, SketchCurve2D};

/// A closed loop of connected curves
#[derive(Clone, Debug)]
pub struct Loop2D {
    curves: Vec<Curve2D>,
}

impl Loop2D {
    /// Create a new loop from curves (validates closure)
    pub fn new(curves: Vec<Curve2D>) -> SketchResult<Self> {
        let loop2d = Self { curves };
        loop2d.validate(HEAL_TOLERANCE)?;
        Ok(loop2d)
    }

    /// Create without validation (use with caution)
    #[allow(dead_code)]
    pub fn new_unchecked(curves: Vec<Curve2D>) -> Self {
        Self { curves }
    }

    /// Create a single-curve loop (must be closed curve like Circle)
    pub fn from_closed_curve(curve: Curve2D) -> SketchResult<Self> {
        if !curve.is_closed(POINT_TOLERANCE) {
            return Err(SketchError::OpenLoop {
                index: 0,
                gap: (curve.end() - curve.start()).magnitude(),
            });
        }
        Ok(Self {
            curves: vec![curve],
        })
    }

    /// Get curves
    pub fn curves(&self) -> &[Curve2D] {
        &self.curves
    }

    /// Get mutable curves (for healing)
    #[allow(dead_code)]
    pub fn curves_mut(&mut self) -> &mut Vec<Curve2D> {
        &mut self.curves
    }

    /// Number of curves in the loop
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.curves.len()
    }

    /// Check if loop is empty
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.curves.is_empty()
    }

    /// Validate that the loop is closed within tolerance
    pub fn validate(&self, tol: f64) -> SketchResult<()> {
        if self.curves.is_empty() {
            return Err(SketchError::EmptyLoop);
        }

        let n = self.curves.len();

        // Single curve: must be closed (like a circle)
        if n == 1 {
            let curve = &self.curves[0];
            let gap = (curve.start() - curve.end()).magnitude();
            if gap > tol {
                return Err(SketchError::OpenLoop { index: 0, gap });
            }
            return Ok(());
        }

        // Multiple curves: each must connect to next
        for i in 0..n {
            let end_pt = self.curves[i].end();
            let start_pt = self.curves[(i + 1) % n].start();
            let gap = (end_pt - start_pt).magnitude();

            if gap > tol {
                return Err(SketchError::OpenLoop { index: i, gap });
            }
        }

        Ok(())
    }

    /// Attempt to heal small gaps by adjusting line endpoints
    #[allow(dead_code)]
    pub fn heal_gaps(&mut self, tol: f64) -> usize {
        let mut healed = 0;
        let n = self.curves.len();

        if n < 2 {
            return 0;
        }

        for i in 0..n {
            let end_pt = self.curves[i].end();
            let next_idx = (i + 1) % n;
            let start_pt = self.curves[next_idx].start();
            let gap = (end_pt - start_pt).magnitude();

            if gap > POINT_TOLERANCE && gap <= tol {
                // Move next curve's start to current curve's end
                self.curves[next_idx].set_start(end_pt);
                healed += 1;
            }
        }

        healed
    }

    /// Total length of all curves in the loop
    #[allow(dead_code)]
    pub fn total_length(&self) -> f64 {
        self.curves.iter().map(|c| c.length()).sum()
    }

    /// Bounding box of the entire loop
    #[allow(dead_code)]
    pub fn bounding_box(&self) -> Option<BoundingBox2D> {
        if self.curves.is_empty() {
            return None;
        }

        let mut bbox = self.curves[0].bounding_box();
        for curve in self.curves.iter().skip(1) {
            bbox = bbox.union(&curve.bounding_box());
        }
        Some(bbox)
    }

    /// Check winding direction (true = CCW, false = CW)
    #[allow(dead_code)]
    pub fn is_ccw(&self) -> bool {
        // Calculate signed area using shoelace formula on sampled points
        let mut area = 0.0;

        for curve in &self.curves {
            const SAMPLES: usize = 10;
            for i in 0..SAMPLES {
                let t0 = i as f64 / SAMPLES as f64;
                let t1 = (i + 1) as f64 / SAMPLES as f64;
                let p0 = curve.point_at(t0);
                let p1 = curve.point_at(t1);
                area += (p1.x - p0.x) * (p1.y + p0.y);
            }
        }

        area < 0.0 // Negative = CCW in standard math coords
    }

    /// Reverse the direction of the loop
    #[allow(dead_code)]
    pub fn reverse(&mut self) {
        self.curves.reverse();
        for curve in &mut self.curves {
            *curve = curve.reversed();
        }
    }

    /// Return a reversed copy
    #[allow(dead_code)]
    pub fn reversed(&self) -> Self {
        let curves: Vec<_> = self.curves.iter().rev().map(|c| c.reversed()).collect();
        Self { curves }
    }
}
