use glam::Vec3;

#[repr( C )]
#[derive( Debug, Clone, Copy )]
pub struct Vertex {
  pub position: Vec3,
  pub normal: Vec3,
  pub uv: [f32; 2],
}
