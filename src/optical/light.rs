use crate::optical::color::RGBColor;
use glam::Vec3;

#[repr( C )]
#[derive( Clone, Copy, Debug )]
pub struct Light {
  pub intensity: f32,
  pub color: RGBColor,
}

pub struct DirectionalLight {
  pub direction: Vec3,
  pub intensity: f32, // Light intensity
  pub color: RGBColor, // Light color
  pub diffuse_coefficient: f32, // Diffuse reflection coefficient
  pub specular_coefficient: f32, // Specular reflection coefficient
  pub shininess: f32, // Shininess factor for specular highlights
}

pub struct PointLight {
  location: Vec3,
  intensity: f32,
  color:  RGBColor,
  angle_of_incidence: f32,
  falloff_rate: f32,
}