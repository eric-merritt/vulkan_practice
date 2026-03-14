use glam::Vec3;


pub fn normal(x: f32, y: f32, z: f32) -> Vec3 {
  Vec3::new(x, y, z).normalize()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn result_is_unit_length() {
    let n = normal(3.0, 4.0, 0.0);
    assert!((n.length() - 1.0).abs() < 1e-6);
  }

  #[test]
  fn preserves_direction() {
    let n = normal(0.0, 5.0, 0.0);
    assert!((n.x).abs() < 1e-6);
    assert!((n.y - 1.0).abs() < 1e-6);
    assert!((n.z).abs() < 1e-6);
  }

  #[test]
  fn normalizes_arbitrary_vector() {
    let n = normal(1.0, 1.0, 1.0);
    let expected = 1.0 / 3.0_f32.sqrt();
    assert!((n.x - expected).abs() < 1e-6);
    assert!((n.y - expected).abs() < 1e-6);
    assert!((n.z - expected).abs() < 1e-6);
  }
}