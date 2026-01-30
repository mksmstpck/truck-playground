use bytemuck::{Pod, Zeroable};
use eframe::wgpu;
use truck_meshalgo::prelude::*;
use truck_modeling::Solid;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x3,  // position
        1 => Float32x3,  // normal
    ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub struct GpuMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl GpuMesh {
    /// Convert a truck Solid to GPU-ready mesh data
    pub fn from_solid(solid: &Solid, tolerance: f64) -> Self {
        // 1. Triangulate the solid
        let polygon_mesh = solid.triangulation(tolerance);

        // 2. Get the raw polygon mesh
        let mesh = polygon_mesh.to_polygon();

        // 3. Extract positions
        let positions = mesh.positions();

        // 4. Compute normals (per-face, then average per-vertex)
        //    truck_meshalgo provides this
        let normals = mesh.normals();

        // 5. Build vertex array
        let vertices: Vec<Vertex> = positions
            .iter()
            .zip(normals.iter())
            .map(|(pos, norm)| Vertex {
                position: [pos.x as f32, pos.y as f32, pos.z as f32],
                normal: [norm.x as f32, norm.y as f32, norm.z as f32],
            })
            .collect();

        // 6. Build index array
        let indices: Vec<u32> = mesh
            .tri_faces()
            .iter()
            .flat_map(|face| face.iter().map(|&idx| idx.pos as u32))
            .collect();

        Self { vertices, indices }
    }
}
