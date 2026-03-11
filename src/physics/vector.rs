use glam::Vec3;

pub trait VectorOps {
  fn magnitude(&self) -> f32;
  fn direction(&self) -> Vec3;
}

impl VectorOps for Vec3 {
  fn magnitude(&self) -> f32 {
    self.length()
  }
  fn direction(&self) -> Vec3 {
    self.normalize_or_zero()
  }
}
