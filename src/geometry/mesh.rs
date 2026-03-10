use crate::geometry::vertex::Vertex;
use crate::material::Material;

pub struct Mesh {
  pub vertices: Vec<Vertex>,
  pub indices: Vec<u32>,
  pub material_ids: Vec<usize>,
  pub materials: Vec<Material>
}

impl Mesh {
  pub fn new(
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    material_ids: Vec<usize>,
    materials: Vec<Material>
  ) -> Self {
    Self { vertices, indices }
  }

  pub fn triangle_count( &self ) {
    self.indices.len() / 3
  }
}
