use truck_geometry::prelude::*;

use crate::sketch::primitives::SketchCurve2D;

#[derive(Clone, Debug)]
pub struct Arc2D {
    pub center: Point2,
    pub radius: f64,
    pub start_angle: f64,
    pub end_angle: f64,
}

impl Arc2D {
    fn point_at(&self, angle: f64) -> Point2 {
        Point2::new(
            self.center.x + self.radius * angle.cos(),
            self.center.y + self.radius * angle.sin(),
        )
    }
}

impl SketchCurve2D for Arc2D {
    fn start(&self) -> Point2 {
        self.point_at(self.start_angle)
    }

    fn end(&self) -> Point2 {
        self.point_at(self.end_angle)
    }
}
