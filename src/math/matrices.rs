use glam::{Mat4, Vec3};
use crate::geometry::position::Position;
use crate::math::quaternions::Quaternion;

pub struct Matrix {
  
}

pub fn model_matrix(position: &Position, rotation: &Quaternion, scale: &Vec3) -> Mat4 {
  Mat4::from_scale_rotation_translation(scale, rotation, position.coords)
}

