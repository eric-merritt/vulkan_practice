pub mod color;
pub mod light;
pub mod camera;
pub use camera::Camera;
pub use color::{RGBAColor, RGBColor};
pub use light::{Light, SpotLight, AmbientLight, PointLight, DirectionalLight, Ray};
