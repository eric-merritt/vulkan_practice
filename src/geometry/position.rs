use glam::Vec3;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Position {
  pub coords: Vec3
}

impl Position {
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self { coords: Vec3::new(x, y, z) }
  }

  pub fn distance_to(&self, other: &Position) -> f32 {
    (other.coords - self.coords).length()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_sets_coords() {
    let p = Position::new(1.0, 2.0, 3.0);
    assert_eq!(p.coords, Vec3::new(1.0, 2.0, 3.0));
  }

  #[test]
  fn distance_to_same_point_is_zero() {
    let p = Position::new(0.0, 0.0, 0.0);
    assert_eq!(p.distance_to(&p), 0.0);
  }

  #[test]
  fn distance_to_known_distance() {
    let a = Position::new(0.0, 0.0, 0.0);
    let b = Position::new(3.0, 4.0, 0.0);
    assert!((a.distance_to(&b) - 5.0).abs() < 1e-6);
  }

  #[test]
  fn distance_is_symmetric() {
    let a = Position::new(1.0, 2.0, 3.0);
    let b = Position::new(4.0, 5.0, 6.0);
    assert!((a.distance_to(&b) - b.distance_to(&a)).abs() < 1e-6);
  }
}