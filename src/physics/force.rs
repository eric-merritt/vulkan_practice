use glam::Vec3;
use crate::Position;

#[derive(Debug, Clone, Copy)]
pub struct Force {
  pub vector: Vec3,
  pub application_point: Option<Position>
}

impl Force {
  pub fn new(vector: Vec3, application_point: Option<Position>) -> Self {
    Self { vector, application_point }
  }
  
  pub fn magnitude(&self) -> f32 {
    self.vector.length()
  }

  pub fn direction(&self) -> Vec3 {
    self.vector.normalize_or_zero()
  }

  pub fn to_acceleration(&self, mass: f32) -> Vec3 {
    self.vector / mass
  }
}