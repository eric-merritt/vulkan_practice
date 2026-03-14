use glam::Vec3;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct AxisAlignedBoundingBox {
  pub min: Vec3,
  pub max: Vec3,
}

impl AxisAlignedBoundingBox {
  pub fn new(min: Vec3, max: Vec3) -> Self { Self {min, max}}
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_sets_min_and_max() {
    let aabb = AxisAlignedBoundingBox::new(Vec3::new(-1.0, -2.0, -3.0), Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(aabb.min, Vec3::new(-1.0, -2.0, -3.0));
    assert_eq!(aabb.max, Vec3::new(1.0, 2.0, 3.0));
  }

  #[test]
  fn is_copy() {
    let a = AxisAlignedBoundingBox::new(Vec3::ZERO, Vec3::ONE);
    let b = a;
    assert_eq!(a.min, b.min);
    assert_eq!(a.max, b.max);
  }
}
