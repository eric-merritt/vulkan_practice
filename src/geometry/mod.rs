use crate::optical::color::RGBAColor;
use glam::Vec3;
pub mod vertex;
pub mod mesh;
pub mod position;
pub use position::Position;
pub use mesh::Mesh;
pub use vertex::Vertex;

#[repr( C )]
#[derive( Debug, Clone, Copy )]
pub struct Pixel {
  pub location: Position,
  pub depth: f32,
  pub color: RGBAColor,
}
