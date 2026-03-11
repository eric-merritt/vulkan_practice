use glam::Vec3;
use crate::geometry::position::Position;

#[repr( C )]
#[derive( Debug, Clone, Copy )]
pub struct Vertex {
  pub position: Position,
  pub normal: Vec3,
  pub uv: [f32; 2],
}
