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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_sets_vector() {
    let v = Velocity::new(Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(v.vector, Vec3::new(1.0, 2.0, 3.0));
  }

  #[test]
  fn speed_is_magnitude() {
    let v = Velocity::new(Vec3::new(3.0, 4.0, 0.0));
    assert!((v.speed() - 5.0).abs() < 1e-6);
  }

  #[test]
  fn speed_of_zero_is_zero() {
    let v = Velocity::new(Vec3::ZERO);
    assert_eq!(v.speed(), 0.0);
  }

  #[test]
  fn direction_is_normalized() {
    let v = Velocity::new(Vec3::new(3.0, 4.0, 0.0));
    let dir = v.direction();
    assert!((dir.length() - 1.0).abs() < 1e-6);
    assert!((dir.x - 0.6).abs() < 1e-6);
    assert!((dir.y - 0.8).abs() < 1e-6);
  }

  #[test]
  fn direction_of_zero_is_zero() {
    let v = Velocity::new(Vec3::ZERO);
    assert_eq!(v.direction(), Vec3::ZERO);
  }

  #[test]
  fn apply_acceleration_integrates() {
    let mut v = Velocity::new(Vec3::new(1.0, 0.0, 0.0));
    let accel = Vec3::new(0.0, -9.8, 0.0);
    v.apply_acceleration(&accel, 1.0);
    assert!((v.vector.x - 1.0).abs() < 1e-6);
    assert!((v.vector.y - -9.8).abs() < 1e-6);
  }

  #[test]
  fn apply_acceleration_scales_by_dt() {
    let mut v = Velocity::new(Vec3::ZERO);
    let accel = Vec3::new(10.0, 0.0, 0.0);
    v.apply_acceleration(&accel, 0.5);
    assert!((v.vector.x - 5.0).abs() < 1e-6);
  }
}