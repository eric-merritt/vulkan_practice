use glam::Vec3;

#[derive(Debug, Copy, Clone)]
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