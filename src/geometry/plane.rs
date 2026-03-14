use glam::Vec3;
use crate::geometry::position::Position;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Plane {
  normal: Vec3,
  distance: f32
}

impl Plane {
  pub fn new(raw_normal: Vec3, raw_distance: f32) -> Self {
    let length = raw_normal.length();
    Self { normal: raw_normal / length, distance: raw_distance / length }
  }
  
  pub fn signed_distance(&self, point: Vec3) -> f32 {
    self.normal.dot(point) + self.distance
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_normalizes_normal() {
    let plane = Plane::new(Vec3::new(0.0, 2.0, 0.0), 4.0);
    assert!((plane.normal.length() - 1.0).abs() < 1e-6);
  }

  #[test]
  fn new_scales_distance_with_normal() {
    let plane = Plane::new(Vec3::new(0.0, 2.0, 0.0), 4.0);
    assert!((plane.distance - 2.0).abs() < 1e-6);
  }

  #[test]
  fn signed_distance_positive_in_front() {
    let plane = Plane::new(Vec3::new(0.0, 1.0, 0.0), 0.0);
    let dist = plane.signed_distance(Vec3::new(0.0, 5.0, 0.0));
    assert!(dist > 0.0);
  }

  #[test]
  fn signed_distance_negative_behind() {
    let plane = Plane::new(Vec3::new(0.0, 1.0, 0.0), 0.0);
    let dist = plane.signed_distance(Vec3::new(0.0, -3.0, 0.0));
    assert!(dist < 0.0);
  }

  #[test]
  fn signed_distance_zero_on_plane() {
    let plane = Plane::new(Vec3::new(0.0, 1.0, 0.0), -5.0);
    let dist = plane.signed_distance(Vec3::new(0.0, 5.0, 0.0));
    assert!(dist.abs() < 1e-6);
  }
}