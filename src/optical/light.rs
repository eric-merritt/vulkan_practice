use crate::optical::color::RGBColor;
use glam::Vec3;

#[repr( C )]
#[derive( Clone, Copy, Debug )]
pub struct Light {
  pub intensity: f32,
  pub color: RGBColor,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DirectionalLight {
  pub direction: Vec3,
  pub intensity: f32, // Light intensity
  pub color: RGBColor, // Light color
  pub diffuse_coefficient: f32, // Diffuse reflection coefficient
  pub specular_coefficient: f32, // Specular reflection coefficient
  pub shininess: f32, // Shininess factor for specular highlights
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PointLight {
  pub location: Vec3,
  pub intensity: f32,
  pub color:  RGBColor,
  pub angle_of_incidence: f32,
  pub falloff_rate: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Ray {
  pub origin: Vec3,
  pub direction: Vec3,
}

impl Ray {
  pub fn new(origin: Vec3, direction: Vec3) -> Self {
    Self  {origin, direction }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn ray_new_sets_fields() {
    let origin = Vec3::new(1.0, 2.0, 3.0);
    let direction = Vec3::new(0.0, 0.0, -1.0);
    let ray = Ray::new(origin, direction);
    assert_eq!(ray.origin, origin);
    assert_eq!(ray.direction, direction);
  }

  #[test]
  fn light_fields_accessible() {
    let light = Light {
      intensity: 0.8,
      color: RGBColor::new(1.0, 1.0, 1.0),
    };
    assert_eq!(light.intensity, 0.8);
  }

  #[test]
  fn directional_light_copy() {
    let dl = DirectionalLight {
      direction: Vec3::new(0.0, -1.0, 0.0),
      intensity: 1.0,
      color: RGBColor::new(1.0, 0.9, 0.8),
      diffuse_coefficient: 0.7,
      specular_coefficient: 0.3,
      shininess: 32.0,
    };
    let dl2 = dl;
    assert_eq!(dl.intensity, dl2.intensity);
  }

  #[test]
  fn point_light_copy() {
    let pl = PointLight {
      location: Vec3::ZERO,
      intensity: 1.0,
      color: RGBColor::new(1.0, 1.0, 1.0),
      angle_of_incidence: 0.5,
      falloff_rate: 1.0,
    };
    let pl2 = pl;
    assert_eq!(pl.intensity, pl2.intensity);
  }
}

