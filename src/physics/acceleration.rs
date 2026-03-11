use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Acceleration {
  pub vector: Vec3,
}

impl Acceleration {
  pub fn new(vector: Vec3) -> Self {
    Self { vector }
  }

  pub fn magnitude(&self) -> f32 {
    self.vector.length()
  }

  pub fn direction(&self) -> Vec3 {
    self.vector.normalize_or_zero()
  }
}