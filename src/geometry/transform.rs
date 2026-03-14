#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Transform {
  position: Position,
  rotation: Quaternion,
  scale: Vec3,
}

impl Transform {
  pub fn new(
    position: Position, 
    rotation: Rotation,  
    scale: Vec3
  ) -> Self {
    Self { position, rotation, scale }
  }
}