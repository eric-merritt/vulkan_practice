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

  }