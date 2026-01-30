use truck_geometry::prelude::*;
use truck_modeling::*;

pub fn create_test_solid() -> Solid {
    // Create a simple box
    let vertex = builder::vertex(Point3::new(-10.0, -10.0, 0.0));
    let edge = builder::tsweep(&vertex, Vector3::new(20.0, 0.0, 0.0));
    let face = builder::tsweep(&edge, Vector3::new(0.0, 20.0, 0.0));
    let solid = builder::tsweep(&face, Vector3::new(0.0, 0.0, 20.0));

    solid
}

pub fn solid_from_sketch(
    sketch: &crate::sketch::Sketch,
    height: f64,
) -> std::result::Result<Solid, crate::sketch::SketchError> {
    let plane = crate::sketch::Plane::xy();
    sketch.extrude(&plane, Vector3::new(0.0, 0.0, height))
}
