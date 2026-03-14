use glam::Vec3;

#[repr(C)]
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_sets_fields() {
    let w = Wave::new(2.0, Vec3::new(1.0, 0.0, 0.0), 3.14, 0.0);
    assert_eq!(w.amplitude, 2.0);
    assert_eq!(w.wave_vector, Vec3::new(1.0, 0.0, 0.0));
    assert_eq!(w.frequency, 3.14);
    assert_eq!(w.phase, 0.0);
  }

  #[test]
  fn is_copy() {
    let w = Wave::new(1.0, Vec3::X, 1.0, 0.0);
    let w2 = w;
    assert_eq!(w.amplitude, w2.amplitude);
  }
}

