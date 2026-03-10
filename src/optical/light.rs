use crate::optical::color::RGBColor;

#[repr( C )]
#[derive( Clone, Copy, Debug )]
pub struct Light {
  pub intensity: f32,
  pub color: RGBColor,
}
