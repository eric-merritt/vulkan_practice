use crate::geometry::position::Position;
use glam::{Quat, Vec3};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Transform {
  pub position: Position,
  pub rotation: Quat,
  pub scale: Vec3,
}

impl Transform {
  pub fn new(
    position: Position, 
    rotation: Quat,  
    scale: Vec3
  ) -> Self {
    Self { position, rotation, scale }
  }
}