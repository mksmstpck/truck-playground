use truck_geometry::prelude::*;

use crate::sketch::primitives::SketchCurve2D;

#[derive(Clone, Debug)]
pub struct Line2D {
    pub start: Point2,
    pub end: Point2,
}

impl SketchCurve2D for Line2D {
    fn start(&self) -> Point2 {
        self.start
    }

    fn end(&self) -> Point2 {
        self.end
    }
}
