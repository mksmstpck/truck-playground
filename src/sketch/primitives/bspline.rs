use crate::sketch::primitives::SketchCurve2D;
use std::ops::Bound;
use truck_geometry::prelude::*;

#[derive(Clone, Debug)]
pub struct BSpline2D {
    pub curve: BSplineCurve<Point2>,
}

impl SketchCurve2D for BSpline2D {
    fn start(&self) -> Point2 {
        let (b0, _) = self.curve.parameter_range();
        let t0 = bound_value(b0);
        self.curve.subs(t0)
    }

    fn end(&self) -> Point2 {
        let (_, b1) = self.curve.parameter_range();
        let t1 = bound_value(b1);
        self.curve.subs(t1)
    }
}

fn bound_value(b: Bound<f64>) -> f64 {
    match b {
        Bound::Included(t) => t,
        Bound::Excluded(t) => t,
        Bound::Unbounded => panic!("unbounded spline parameter"),
    }
}
