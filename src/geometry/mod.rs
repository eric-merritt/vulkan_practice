use crate::optical::color::RGBAColor;
use glam::Vec3;
pub mod vertex;
pub mod mesh;


#[repr( C )]
#[derive( Debug, Clone, Copy )]
pub struct Pixel {
  pub location: Vec3,
  pub depth: f32,
  pub color: RGBAColor,
}
