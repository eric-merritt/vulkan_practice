use glam::Vec3;

#[repr(C)]
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_sets_vector() {
    let a = Acceleration::new(Vec3::new(0.0, -9.8, 0.0));
    assert_eq!(a.vector, Vec3::new(0.0, -9.8, 0.0));
  }

  #[test]
  fn magnitude_is_length() {
    let a = Acceleration::new(Vec3::new(3.0, 4.0, 0.0));
    assert!((a.magnitude() - 5.0).abs() < 1e-6);
  }

  #[test]
  fn direction_is_normalized() {
    let a = Acceleration::new(Vec3::new(0.0, -9.8, 0.0));
    let dir = a.direction();
    assert!((dir.length() - 1.0).abs() < 1e-6);
    assert!((dir.y - -1.0).abs() < 1e-6);
  }

  #[test]
  fn direction_of_zero_is_zero() {
    let a = Acceleration::new(Vec3::ZERO);
    assert_eq!(a.direction(), Vec3::ZERO);
  }
}