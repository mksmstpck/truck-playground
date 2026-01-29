use truck_geometry::prelude::*;
use truck_meshalgo::prelude::*;
use truck_modeling::Solid;
use truck_playground::{Plane, Shapes, Sketch, SketchBuilder};
use truck_stepio::out::{CompleteStepDisplay, StepModel};

fn main() {
    println!("=== Truck Playground - Sketch System Demo ===\n");

    // Example 1: Simple rectangle extrusion
    println!("1. Creating extruded rectangle...");
    let rect = Shapes::rectangle(Point2::new(-5.0, -2.5), 10.0, 5.0).unwrap();
    let sketch = Sketch::new(rect);
    let plane = Plane::xy();
    let box_solid = sketch
        .extrude(&plane, Vector3::new(0.0, 0.0, 3.0))
        .unwrap();
    write_outputs(&box_solid, "rectangle");
    println!("   ✓ Saved rectangle.obj and rectangle.step\n");

    // Example 2: Circle with hole (tube)
    println!("2. Creating tube (circle with hole)...");
    let outer = Shapes::circle(Point2::origin(), 10.0).unwrap();
    let hole = Shapes::circle(Point2::origin(), 7.0).unwrap();
    let sketch = Sketch::with_holes(outer, vec![hole]);
    let tube = sketch
        .extrude(&plane, Vector3::new(0.0, 0.0, 20.0))
        .unwrap();
    write_outputs(&tube, "tube");
    println!("   ✓ Saved tube.obj and tube.step\n");

    // Example 3: Rounded rectangle
    println!("3. Creating rounded rectangle extrusion...");
    let rounded = Shapes::rounded_rectangle(Point2::new(-15.0, -7.5), 30.0, 15.0, 3.0).unwrap();
    let sketch = Sketch::new(rounded);
    let rounded_box = sketch
        .extrude(&plane, Vector3::new(0.0, 0.0, 5.0))
        .unwrap();
    write_outputs(&rounded_box, "rounded_rect");
    println!("   ✓ Saved rounded_rect.obj and rounded_rect.step\n");

    // Example 4: Custom profile with builder
    println!("4. Creating custom L-shaped profile...");
    let l_shape = SketchBuilder::new()
        .move_to(Point2::new(0.0, 0.0))
        .horizontal(20.0)
        .unwrap()
        .vertical(5.0)
        .unwrap()
        .horizontal(-15.0)
        .unwrap()
        .vertical(15.0)
        .unwrap()
        .horizontal(-5.0)
        .unwrap()
        .close()
        .unwrap();

    let sketch = Sketch::new(l_shape);
    let l_extrusion = sketch
        .extrude(&plane, Vector3::new(0.0, 0.0, 8.0))
        .unwrap();
    write_outputs(&l_extrusion, "l_shape");
    println!("   ✓ Saved l_shape.obj and l_shape.step\n");

    // Example 5: Slot shape
    println!("5. Creating slot extrusion...");
    let slot = Shapes::slot(Point2::origin(), 30.0, 10.0, true).unwrap();
    let sketch = Sketch::new(slot);
    let slot_solid = sketch
        .extrude(&plane, Vector3::new(0.0, 0.0, 4.0))
        .unwrap();
    write_outputs(&slot_solid, "slot");
    println!("   ✓ Saved slot.obj and slot.step\n");

    // Example 6: Hexagon
    println!("6. Creating hexagonal prism...");
    let hex = Shapes::hexagon(Point2::origin(), 12.0).unwrap();
    let sketch = Sketch::new(hex);
    let hex_prism = sketch
        .extrude(&plane, Vector3::new(0.0, 0.0, 10.0))
        .unwrap();
    write_outputs(&hex_prism, "hexagon");
    println!("   ✓ Saved hexagon.obj and hexagon.step\n");

    println!("=== All models generated successfully! ===");
}

fn write_outputs(solid: &Solid, name: &str) {
    write_polygon_to_file(solid, &format!("{}.obj", name));
    save_step(solid, &format!("{}.step", name));
}

fn write_polygon_to_file(solid: &Solid, path: &str) {
    let mesh_with_topology = solid.triangulation(0.01);
    let mesh = mesh_with_topology.to_polygon();

    let mut obj = std::fs::File::create(path).unwrap();
    obj::write(&mesh, &mut obj).unwrap();
}

fn save_step(solid: &Solid, path: &str) {
    let compressed = solid.compress();
    let display = CompleteStepDisplay::new(StepModel::from(&compressed), Default::default());
    let step_string: String = display.to_string();
    std::fs::write(path, &step_string).unwrap();
}
