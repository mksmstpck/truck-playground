use crate::sketch::primitives::{Curve2D, Loop2D, SketchCurve2D};
use truck_geometry::prelude::*;

pub struct Sketch {
    pub outer: Loop2D,
    pub holes: Vec<Loop2D>,
}

pub struct Plane {
    pub origin: Point3,
    pub x_dir: Vector3,
    pub y_dir: Vector3,
}

impl Plane {
    pub fn to_truck_plane(&self) -> truck_geometry::specifieds::Plane {
        // Calculate normal (must not be zero; x_dir and y_dir must be non-collinear)
        let normal = self.x_dir.cross(self.y_dir).normalize();

        //FIX: add propper error handling
        if normal == Vector3::new(0.0, 0.0, 0.0) {
            print!("error here, normal is zero")
        }

        // Choose three points on the plane:
        // - origin
        // - origin + x_dir
        // - origin + y_dir
        let p0 = self.origin;
        let p1 = self.origin + self.x_dir;
        let p2 = self.origin + self.y_dir;

        // Construct Truck's Plane from three points.
        truck_geometry::specifieds::Plane::new(p0, p1, p2)
    }

    pub fn lift_point(&self, p: Point2) -> Point3 {
        self.origin + self.x_dir * p.x + self.y_dir * p.y
    }
}

pub struct CircleArc3 {
    pub center: Point3,
    pub normal: Vector3,
    pub start: Point3,
    pub end: Point3,
}

pub enum Curve3D {
    Line(Line<Point3>),
    Arc(CircleArc3),
    BSpline(BSplineCurve<Point3>),
}

impl Curve2D {
    pub fn to_curve3d(&self, plane: &Plane) -> Curve3D {
        match self {
            Curve2D::Line(line) => {
                let p0 = plane.lift_point(line.start);
                let p1 = plane.lift_point(line.end);
                Curve3D::Line(Line::from_origin_direction(p0, p1 - p0))
            }
            Curve2D::Arc(arc) => {
                let start3d = plane.lift_point(arc.start());
                let end3d = plane.lift_point(arc.end());
                let center3d = plane.lift_point(arc.center);
                let normal = plane.x_dir.cross(plane.y_dir);
                Curve3D::Arc(CircleArc3 {
                    center: center3d,
                    normal,
                    start: start3d,
                    end: end3d,
                })
            }
            Curve2D::BSpline(bspline) => {
                let lifted_pts: Vec<Point3> = bspline
                    .curve
                    .control_points()
                    .iter()
                    .map(|&p| plane.lift_point(p))
                    .collect();
                let degree = bspline.curve.degree();
                let n = lifted_pts.len();
                let knots = KnotVec::uniform_knot(n, degree);
                let lifted_bspline = BSplineCurve::new(knots, lifted_pts);
                Curve3D::BSpline(lifted_bspline)
            }
        }
    }
}

pub mod primitives;
