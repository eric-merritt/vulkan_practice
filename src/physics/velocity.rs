use glam::Vec3;
use crate::geometry::position::Position;

#[repr(C)]
#[derive(Debug,Clone,Copy)]
pub struct Velocity {
  pub vector: Vec3,
}


impl Velocity {
  
  pub fn new(vector: Vec3) -> Self {
    Self { vector }
  }

  pub fn speed(&self) -> f32 {
    self.vector.length()
  }

  pub fn direction(&self) -> Vec3 {
    self.vector.normalize_or_zero()
  }

  pub fn apply_acceleration(&mut self, accel: &Vec3, dt: f32) {
    self.vector += *accel * dt;
  }

}