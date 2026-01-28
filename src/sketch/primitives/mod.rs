use crate::sketch::primitives::arc2d::Arc2D;
use crate::sketch::primitives::bspline::BSpline2D;
use crate::sketch::primitives::line2d::Line2D;
use crate::sketch::{self, Curve3D};
use truck_geometry::prelude::*;

#[derive(Clone, Debug)]
pub enum Curve2D {
    Line(Line2D),
    Arc(Arc2D),
    BSpline(BSpline2D),
}

impl SketchCurve2D for Curve2D {
    fn start(&self) -> Point2 {
        match self {
            Curve2D::Line(c) => c.start(),
            Curve2D::Arc(c) => c.start(),
            Curve2D::BSpline(c) => c.start(),
        }
    }

    fn end(&self) -> Point2 {
        match self {
            Curve2D::Line(c) => c.end(),
            Curve2D::Arc(c) => c.end(),
            Curve2D::BSpline(c) => c.end(),
        }
    }
}

pub trait SketchCurve2D {
    fn start(&self) -> Point2;
    fn end(&self) -> Point2;
}

#[derive(Clone)]
pub struct Loop2D {
    pub curves: Vec<Curve2D>,
}

impl Loop2D {
    pub fn validate_closed_loop(&self, tol: f64) -> bool {
        let n = self.curves.len();
        if n < 2 {
            return false;
        }

        for i in 0..n {
            let a = self.curves[i].end();
            let b = self.curves[(i + 1) % n].start();

            if (a - b).magnitude() > tol {
                return false;
            }
        }

        true
    }

    pub fn to_wire3d(&self, plane: &sketch::Plane) -> Vec<Curve3D> {
        self.curves.iter().map(|c| c.to_curve3d(plane)).collect()
    }
}

pub mod arc2d;
pub mod bspline;
pub mod line2d;
