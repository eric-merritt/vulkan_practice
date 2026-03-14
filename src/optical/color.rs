#[repr( C )]
#[derive( Clone, Copy, Debug )]
pub struct RGBColor {
  pub r: f32,
  pub g: f32,
  pub b: f32
}

#[repr( C )]
#[derive( Clone, Copy, Debug )]
pub struct RGBAColor {
  pub r: f32,
  pub g: f32,
  pub b: f32,
  pub a: f32
}

impl RGBColor {
  pub fn new(r: f32, g: f32, b: f32) -> Self {
    RGBColor { r, g, b }
  }
}

impl RGBAColor {
  pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
    RGBAColor { r, g, b, a }
  }
}
