use truck_meshalgo::prelude::*;
use truck_modeling::Edge;
use truck_modeling::Face;
use truck_modeling::Solid;
use truck_modeling::Vertex;
use truck_modeling::builder;
use truck_stepio::out::CompleteStepDisplay;
use truck_stepio::out::StepModel;

fn main() {
    let cube = cube();

    write_polygon_to_file(&cube, "cube.obj");
    save_step(&cube, "cube.step");
}

fn write_polygon_to_file(solid: &Solid, path: &str) {
    let mesh_with_topology = solid.triangulation(0.001);
    let mesh = mesh_with_topology.to_polygon();

    let mut obj = std::fs::File::create(path).unwrap();

    obj::write(&mesh, &mut obj).unwrap();
}

fn save_step(solid: &Solid, path: &str) {
    // compress solid data.
    let compressed = solid.compress();
    // step format display
    let display = CompleteStepDisplay::new(StepModel::from(&compressed), Default::default());
    // content of step file
    let step_string: String = display.to_string();
    std::fs::write(path, &step_string).unwrap();
}

fn cube() -> Solid {
    let vertex: Vertex = builder::vertex(Point3::new(-1.0, 0.0, -1.0));
    let edge: Edge = builder::tsweep(&vertex, 2.0 * Vector3::unit_z());
    let face: Face = builder::tsweep(&edge, 2.0 * Vector3::unit_x());
    builder::tsweep(&face, 2.0 * Vector3::unit_y())
}
