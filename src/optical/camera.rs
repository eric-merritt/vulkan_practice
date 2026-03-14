use crate::geometry::transform::Transform;
use crate::geometry::position::Position;
use glam::{Vec3, Mat4, Quat};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Camera {
  pub transform: Transform,
  pub fov: f32,
  pub aspect: f32,
  pub near: f32,
  pub far: f32
}

impl Camera {
  pub fn view_matrix(&self) -> Mat4 {
    let eye = self.transform.position.coords;
    let target = eye + self.transform.rotation * Vec3::NEG_Z;
    let up = self.transform.rotation * Vec3::Y;

    Mat4::look_at_rh(eye, target, up)
  }

  pub fn projection_matrix(&self) -> Mat4 {
    Mat4::perspective_rh(self.fov, self.aspect, self.near, self.far)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::f32::consts::FRAC_PI_4;

  fn default_camera() -> Camera {
    Camera {
      transform: Transform::new(
        Position::new(0.0, 0.0, 5.0),
        Quat::IDENTITY,
        Vec3::ONE,
      ),
      fov: FRAC_PI_4,
      aspect: 16.0 / 9.0,
      near: 0.1,
      far: 100.0,
    }
  }

  #[test]
  fn view_matrix_is_not_identity() {
    let cam = default_camera();
    let view = cam.view_matrix();
    assert_ne!(view, Mat4::IDENTITY);
  }

  #[test]
  fn view_matrix_translates_eye_to_origin() {
    let cam = default_camera();
    let view = cam.view_matrix();
    // transform the eye position by the view matrix — should land near origin
    let eye_in_view = view.transform_point3(cam.transform.position.coords);
    assert!(eye_in_view.length() < 1e-4);
  }

  #[test]
  fn projection_matrix_is_not_identity() {
    let cam = default_camera();
    let proj = cam.projection_matrix();
    assert_ne!(proj, Mat4::IDENTITY);
  }

  #[test]
  fn projection_matrix_changes_with_fov() {
    let cam1 = default_camera();
    let mut cam2 = default_camera();
    cam2.fov = FRAC_PI_4 * 2.0;
    assert_ne!(cam1.projection_matrix(), cam2.projection_matrix());
  }

  #[test]
  fn projection_matrix_changes_with_aspect() {
    let cam1 = default_camera();
    let mut cam2 = default_camera();
    cam2.aspect = 4.0 / 3.0;
    assert_ne!(cam1.projection_matrix(), cam2.projection_matrix());
  }

  #[test]
  fn rotated_camera_changes_view() {
    let cam1 = default_camera();
    let mut cam2 = default_camera();
    cam2.transform.rotation = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
    assert_ne!(cam1.view_matrix(), cam2.view_matrix());
  }
}
