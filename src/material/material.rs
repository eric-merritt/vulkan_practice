use crate::optical::color::RGBAColor;

#[derive( Debug )]
pub struct Material {
  pub name: String,
  pub diffuse_color: RGBAColor,
  pub specular_color: RGBAColor,
  pub shininess: f32,
  pub texture_id: Option<usize>
}

impl Material {
  pub fn new(
    name: &str,
    diffuse_color: RGBAColor,
    specular_color: RGBAColor,
    shininess: f32,
    texture_id: Option<usize>,
  ) -> Self {
    Self {
      name: name.to_string(),
      diffuse_color,
      specular_color,
      shininess,
      texture_id,
    }
  }
}
