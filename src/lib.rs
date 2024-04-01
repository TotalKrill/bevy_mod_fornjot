use bevy::{prelude::Mesh, render::render_asset::RenderAssetUsages};
pub use fj_core::{
    self,
    algorithms::{approx::Tolerance, triangulate::Triangulate},
};
pub use fj_interop;
pub use fj_math;
use fj_math::Point;

pub trait ToBevyMesh {
    fn to_bevy_mesh(self, generate_normals: bool) -> Mesh;
}

impl ToBevyMesh for fj_interop::Mesh<Point<3>> {
    fn to_bevy_mesh(self, generate_normals: bool) -> Mesh {
        // translate into bevy coords
        let vertices: Vec<bevy::math::Vec3> = self
            .vertices()
            .into_iter()
            .map(|point| {
                bevy::math::Vec3::new(point.x.into_f32(), point.z.into_f32(), -point.y.into_f32())
            })
            .collect();
        let indices: Vec<u32> = self.indices().collect();
        let mut mesh = Mesh::new(
            bevy::render::render_resource::PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
        .with_inserted_indices(bevy::render::mesh::Indices::U32(indices));
        if generate_normals {
            mesh.duplicate_vertices();
            mesh.compute_flat_normals();
        }

        mesh
    }
}
