use crate::geometry::position::Position;

pub struct Sphere {
  pub center: Position,
  pub radius: f32,
}

impl Sphere {
  pub fn new(&self, center: Position, radius: f32) -> Self {
    Self { center, radius }
  }
}