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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_sets_fields() {
    let pos = Position::new(1.0, 2.0, 3.0);
    let f = Force::new(Vec3::new(10.0, 0.0, 0.0), Some(pos));
    assert_eq!(f.vector, Vec3::new(10.0, 0.0, 0.0));
    assert!(f.application_point.is_some());
  }

  #[test]
  fn new_with_no_application_point() {
    let f = Force::new(Vec3::new(0.0, -9.8, 0.0), None);
    assert!(f.application_point.is_none());
  }

  #[test]
  fn magnitude_is_length() {
    let f = Force::new(Vec3::new(3.0, 4.0, 0.0), None);
    assert!((f.magnitude() - 5.0).abs() < 1e-6);
  }

  #[test]
  fn direction_is_normalized() {
    let f = Force::new(Vec3::new(0.0, 10.0, 0.0), None);
    let dir = f.direction();
    assert!((dir.length() - 1.0).abs() < 1e-6);
    assert!((dir.y - 1.0).abs() < 1e-6);
  }

  #[test]
  fn to_acceleration_divides_by_mass() {
    let f = Force::new(Vec3::new(10.0, 0.0, 0.0), None);
    let a = f.to_acceleration(2.0);
    assert!((a.x - 5.0).abs() < 1e-6);
  }
}