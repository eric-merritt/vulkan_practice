use crate::geometry::position::Position;
use crate::optical::color::RGBAColor;

#[repr( C )]
#[derive( Debug, Clone, Copy )]
pub struct Pixel {
  pub location: Position,
  pub depth: f32,
  pub color: RGBAColor,
}