use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Wave {
  /// Maximum amplitude of the wave (scalar)
  pub amplitude: f32,

  /// direction of the wave * wavenumber
  pub wave_vector: Vec3,

  // angular frequency in radians per second
  pub frequency: f32,
  
  // phase offset in radians
  pub phase: f32,

}

impl Wave {
  fn new(
    amplitude: f32, 
    wave_vector: Vec3, 
    frequency: f32, 
    phase: f32
  ) -> Self {
    Self { 
      amplitude, 
      wave_vector, 
      frequency, 
      phase 
    }
  }
}

