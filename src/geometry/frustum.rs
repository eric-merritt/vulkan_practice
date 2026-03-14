use glam::{Vec3, Mat4};
use crate::geometry::plane::Plane;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Frustum {
  planes: [Plane; 6]
}


impl Frustum {
  
  pub fn from_view_projection(vp: Mat4) -> Self {
    let cols = vp.to_cols_array_2d();
    let row1 = (cols[0][0], cols[1][0], cols[2][0], cols[3][0]);
    let row2 = (cols[0][1], cols[1][1], cols[2][1], cols[3][1]);
    let row3 = (cols[0][2], cols[1][2], cols[2][2], cols[3][2]);
    let row4 = (cols[0][3], cols[1][3], cols[2][3], cols[3][3]);
    
    let left: Plane = Plane::new(Vec3::new(row4.0 + row1.0, row4.1 + row1.1, row4.2 + row1.2), row4.3 + row1.3);
    let right: Plane = Plane::new(Vec3::new(row4.0 - row1.0, row4.1 - row1.1, row4.2 - row1.2), row4.3 - row1.3);
    let bottom: Plane = Plane::new(Vec3::new(row4.0 + row2.0, row4.1 + row2.1, row4.2 + row2.2), row4.3 + row2.3);
    let top: Plane = Plane::new(Vec3::new(row4.0 - row2.0, row4.1 - row2.1, row4.2 - row2.2), row4.3 - row2.3);
    let near: Plane = Plane::new(Vec3::new(row4.0 + row3.0, row4.1 + row3.1, row4.2 + row3.2), row4.3 + row3.3);
    let far: Plane = Plane::new(Vec3::new(row4.0 - row3.0, row4.1 - row3.1, row4.2 - row3.2), row4.3 - row3.3);

    let planes = [left, right, bottom, top, near, far];
    Self {planes}
  }

  pub fn contains_point(&self, point: Vec3) -> bool {
    for plane in self.planes.iter() {
      if plane.signed_distance(point) < 0.0 {
        return false;
      }
    }
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f32::consts::FRAC_PI_4;

  fn test_vp() -> Mat4 {
    let view = Mat4::look_at_rh(
      Vec3::new(0.0, 0.0, 5.0),
      Vec3::ZERO,
      Vec3::Y,
    );
    let proj = Mat4::perspective_rh(FRAC_PI_4, 1.0, 0.1, 100.0);
    proj * view
  }

  #[test]
  fn origin_is_inside() {
    let frustum = Frustum::from_view_projection(test_vp());
    assert!(frustum.contains_point(Vec3::ZERO));
  }

  #[test]
  fn point_in_front_of_camera_is_inside() {
    let frustum = Frustum::from_view_projection(test_vp());
    assert!(frustum.contains_point(Vec3::new(0.0, 0.0, -1.0)));
  }

  #[test]
  fn point_behind_camera_is_outside() {
    let frustum = Frustum::from_view_projection(test_vp());
    assert!(!frustum.contains_point(Vec3::new(0.0, 0.0, 10.0)));
  }

  #[test]
  fn point_far_left_is_outside() {
    let frustum = Frustum::from_view_projection(test_vp());
    assert!(!frustum.contains_point(Vec3::new(-1000.0, 0.0, 0.0)));
  }

  #[test]
  fn point_far_right_is_outside() {
    let frustum = Frustum::from_view_projection(test_vp());
    assert!(!frustum.contains_point(Vec3::new(1000.0, 0.0, 0.0)));
  }

  #[test]
  fn point_far_above_is_outside() {
    let frustum = Frustum::from_view_projection(test_vp());
    assert!(!frustum.contains_point(Vec3::new(0.0, 1000.0, 0.0)));
  }

  #[test]
  fn point_beyond_far_plane_is_outside() {
    let frustum = Frustum::from_view_projection(test_vp());
    assert!(!frustum.contains_point(Vec3::new(0.0, 0.0, -200.0)));
  }
}
