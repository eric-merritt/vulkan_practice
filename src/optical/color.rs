#[repr( C )]
#[derive( Clone, Copy, Debug )]
pub struct RGBColor {
  r: f32,
  g: f32,
  b: f32
}

#[repr( C )]
#[derive( Clone, Copy, Debug )]
pub struct RGBAColor {
  r: f32,
  g: f32,
  b: f32,
  a: f32
}
