use crate::sketch::error::*;
use crate::sketch::loop2d::Loop2D;
use crate::sketch::plane::Plane;
use crate::sketch::primitives::{Arc2D, BSpline2D, Circle2D, Curve2D, Line2D, SketchCurve2D};
use std::f64::consts::PI;
use truck_geometry::prelude::*;
use truck_modeling::{builder, Curve, Edge, Vertex, Wire};

impl Loop2D {
    /// Convert to truck Wire
    pub fn to_truck_wire(&self, plane: &Plane) -> SketchResult<Wire> {
        let curves = self.curves();
        if curves.is_empty() {
            return Err(SketchError::EmptyLoop);
        }

        // For single closed curve (like a circle)
        if curves.len() == 1 {
            if let Curve2D::Circle(circle) = &curves[0] {
                return circle_to_wire(circle, plane);
            } else {
                return Err(SketchError::OpenLoop {
                    index: 0,
                    gap: (curves[0].end() - curves[0].start()).magnitude(),
                });
            }
        }

        // Create shared vertices for all connection points
        let mut vertices: Vec<Vertex> = Vec::with_capacity(curves.len());
        for curve in curves {
            let pt = plane.lift_point(curve.start());
            vertices.push(builder::vertex(pt));
        }

        // Build edges using shared vertices
        let mut edges: Vec<Edge> = Vec::with_capacity(curves.len());
        let n = curves.len();

        for i in 0..n {
            let v0 = &vertices[i];
            let v1 = &vertices[(i + 1) % n];
            let edge = curve_to_edge_with_vertices(&curves[i], plane, v0, v1)?;
            edges.push(edge);
        }

        let wire: Wire = edges.into_iter().collect();
        Ok(wire)
    }
}

/// Convert curve to edge using pre-created shared vertices
fn curve_to_edge_with_vertices(
    curve: &Curve2D,
    plane: &Plane,
    v0: &Vertex,
    v1: &Vertex,
) -> SketchResult<Edge> {
    match curve {
        Curve2D::Line(line) => line_to_edge_with_vertices(line, plane, v0, v1),
        Curve2D::Arc(arc) => arc_to_edge_with_vertices(arc, plane, v0, v1),
        Curve2D::Circle(_) => {
            // Full circles should only appear as single-curve loops
            // and are handled separately in to_truck_wire
            Err(SketchError::TruckEdgeError(
                "Circle cannot be part of a multi-curve loop".to_string(),
            ))
        }
        Curve2D::BSpline(spline) => bspline_to_edge_with_vertices(spline, plane, v0, v1),
    }
}

fn line_to_edge_with_vertices(
    _line: &Line2D,
    _plane: &Plane,
    v0: &Vertex,
    v1: &Vertex,
) -> SketchResult<Edge> {
    Ok(builder::line(v0, v1))
}

fn arc_to_edge_with_vertices(
    arc: &Arc2D,
    plane: &Plane,
    v0: &Vertex,
    v1: &Vertex,
) -> SketchResult<Edge> {
    let start3d = plane.lift_point(arc.start());
    let center3d = plane.lift_point(arc.center());
    let normal = plane.normal();

    // Create NURBS representation of arc
    let nurbs = arc_to_nurbs(center3d, normal, start3d, arc.sweep_angle())?;

    Edge::try_new(v0, v1, Curve::NurbsCurve(nurbs))
        .map_err(|e| SketchError::TruckEdgeError(format!("{:?}", e)))
}

/// Convert a single circle to a wire (two semicircular edges)
fn circle_to_wire(circle: &Circle2D, plane: &Plane) -> SketchResult<Wire> {
    let center3d = plane.lift_point(circle.center());
    let start3d = plane.lift_point(circle.start());
    let normal = plane.normal();
    
    // Calculate opposite point on circle
    let radius = circle.radius();
    let x_axis = (start3d - center3d).normalize();
    let opposite3d = center3d - x_axis * radius;
    
    // Create two shared vertices
    let v0 = builder::vertex(start3d);
    let v1 = builder::vertex(opposite3d);
    
    let half_sweep = if circle.is_ccw() {
        std::f64::consts::PI
    } else {
        -std::f64::consts::PI
    };
    
    // First semicircle: start -> opposite
    let nurbs1 = arc_to_nurbs(center3d, normal, start3d, half_sweep)?;
    let edge1 = Edge::try_new(&v0, &v1, Curve::NurbsCurve(nurbs1))
        .map_err(|e| SketchError::TruckEdgeError(format!("{:?}", e)))?;
    
    // Second semicircle: opposite -> start
    let nurbs2 = arc_to_nurbs(center3d, normal, opposite3d, half_sweep)?;
    let edge2 = Edge::try_new(&v1, &v0, Curve::NurbsCurve(nurbs2))
        .map_err(|e| SketchError::TruckEdgeError(format!("{:?}", e)))?;
    
    let wire: Wire = vec![edge1, edge2].into_iter().collect();
    Ok(wire)
}

fn bspline_to_edge_with_vertices(
    spline: &BSpline2D,
    plane: &Plane,
    v0: &Vertex,
    v1: &Vertex,
) -> SketchResult<Edge> {
    // Lift control points
    let lifted_pts: Vec<Point3> = spline
        .control_points()
        .iter()
        .map(|&p| plane.lift_point(p))
        .collect();

    let inner = spline.inner();
    let degree = inner.degree();
    let n = lifted_pts.len();
    let knots = KnotVec::uniform_knot(n, degree);
    let lifted_bspline = BSplineCurve::new(knots, lifted_pts);

    Edge::try_new(v0, v1, Curve::BSplineCurve(lifted_bspline))
        .map_err(|e| SketchError::TruckEdgeError(format!("{:?}", e)))
}

/// Create NURBS arc (rational B-spline for circular arcs)
fn arc_to_nurbs(
    center: Point3,
    normal: Vector3,
    start: Point3,
    sweep_angle: f64,
) -> SketchResult<NurbsCurve<Vector4>> {
    let radius = (start - center).magnitude();
    let x_axis = (start - center).normalize();
    let y_axis = normal.cross(x_axis).normalize();

    // Number of segments (each segment is up to 90 degrees)
    let n_segments = ((sweep_angle.abs() / (PI / 2.0)).ceil() as usize).max(1);
    let segment_angle = sweep_angle / n_segments as f64;

    let mut control_points = Vec::new();
    let mut knots = vec![0.0, 0.0, 0.0];

    let w1 = (segment_angle.abs() / 2.0).cos();

    for i in 0..n_segments {
        let theta0 = i as f64 * segment_angle;
        let theta1 = (i + 1) as f64 * segment_angle;
        let theta_mid = (theta0 + theta1) / 2.0;

        let p0 = center + radius * (theta0.cos() * x_axis + theta0.sin() * y_axis);
        let p2 = center + radius * (theta1.cos() * x_axis + theta1.sin() * y_axis);

        let r_mid = radius / w1;
        let p1 = center + r_mid * (theta_mid.cos() * x_axis + theta_mid.sin() * y_axis);

        if i == 0 {
            control_points.push(Vector4::new(p0.x, p0.y, p0.z, 1.0));
        }

        control_points.push(Vector4::new(p1.x * w1, p1.y * w1, p1.z * w1, w1));
        control_points.push(Vector4::new(p2.x, p2.y, p2.z, 1.0));

        let knot_val = (i + 1) as f64 / n_segments as f64;
        knots.extend_from_slice(&[knot_val, knot_val]);
    }

    knots.push(1.0);

    Ok(NurbsCurve::new(BSplineCurve::new(
        KnotVec::from(knots),
        control_points,
    )))
}
