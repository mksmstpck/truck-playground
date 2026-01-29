pub mod builder;
pub mod constants;
pub mod error;
pub mod loop2d;
pub mod plane;
pub mod primitives;
pub mod shapes;
pub mod topology;

pub use builder::SketchBuilder;
pub use error::{SketchError, SketchResult};
pub use loop2d::Loop2D;
pub use plane::Plane;
pub use primitives::{Arc2D, BSpline2D, Circle2D, Curve2D, Line2D, SketchCurve2D};
pub use shapes::Shapes;

use truck_geometry::prelude::*;
use truck_modeling::{builder as truck_builder, Face, Solid, Surface, Wire};

/// A complete sketch with outer boundary and optional holes
pub struct Sketch {
    pub outer: Loop2D,
    pub holes: Vec<Loop2D>,
}

impl Sketch {
    /// Create sketch with just outer boundary
    pub fn new(outer: Loop2D) -> Self {
        Self {
            outer,
            holes: Vec::new(),
        }
    }

    /// Create sketch with holes
    pub fn with_holes(outer: Loop2D, holes: Vec<Loop2D>) -> Self {
        Self { outer, holes }
    }

    /// Add a hole
    #[allow(dead_code)]
    pub fn add_hole(&mut self, hole: Loop2D) {
        self.holes.push(hole);
    }

    /// Convert to truck Wire (outer boundary only)
    #[allow(dead_code)]
    pub fn to_truck_wire(&self, plane: &Plane) -> SketchResult<Wire> {
        self.outer.to_truck_wire(plane)
    }

    /// Convert to truck Face
    pub fn to_truck_face(&self, plane: &Plane) -> SketchResult<Face> {
        let truck_plane = plane.to_truck_plane()?;
        let outer_wire = self.outer.to_truck_wire(plane)?;

        // Create face from outer wire
        let mut face = Face::try_new(vec![outer_wire], Surface::Plane(truck_plane))
            .map_err(|e| SketchError::TruckFaceError(format!("{:?}", e)))?;

        // Add holes
        for hole in &self.holes {
            let hole_wire = hole.to_truck_wire(plane)?;
            face.add_boundary(hole_wire);
        }

        Ok(face)
    }

    /// Extrude sketch into a solid
    pub fn extrude(&self, plane: &Plane, direction: Vector3) -> SketchResult<Solid> {
        let face = self.to_truck_face(plane)?;
        Ok(truck_builder::tsweep(&face, direction))
    }

    /// Revolve sketch into a solid
    #[allow(dead_code)]
    pub fn revolve(
        &self,
        plane: &Plane,
        axis_origin: Point3,
        axis_direction: Vector3,
        angle: Rad<f64>,
    ) -> SketchResult<Solid> {
        let face = self.to_truck_face(plane)?;
        Ok(truck_builder::rsweep(
            &face,
            axis_origin,
            axis_direction,
            angle,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_extrusion() {
        let rect = Shapes::rectangle(Point2::origin(), 10.0, 5.0).unwrap();
        let sketch = Sketch::new(rect);
        let plane = Plane::xy();
        let solid = sketch.extrude(&plane, Vector3::new(0.0, 0.0, 2.0));
        assert!(solid.is_ok());
    }

    #[test]
    fn test_circle_with_hole() {
        let outer = Shapes::circle(Point2::origin(), 50.0).unwrap();
        let hole = Shapes::circle(Point2::origin(), 20.0).unwrap();
        let sketch = Sketch::with_holes(outer, vec![hole]);
        let plane = Plane::xy();
        let solid = sketch.extrude(&plane, Vector3::unit_z() * 10.0);
        assert!(solid.is_ok());
    }
}
