mod geometry;
mod physics;
mod optical;
mod material;
mod engine;

use physics::{Velocity, Acceleration, Force/* , Wave */ };
use geometry::{Position, /*Mesh, Vertex,*/ Pixel};
use optical::color::{ RGBAColor, RGBColor };
use glam::{Vec3, Mat4};
use optical::{Camera, Light, DirectionalLight};
use material::{Material, /*Texture*/};
use engine::{RenderEngine, /*Shader*/}; // Assuming Shader is defined in engine module
use core::f32::consts;


fn main() {
    println!("--- Testing Geometry ---");

    // Position
    let pos = Position::new(1.0, 2.0, 3.0);
    println!("Position: {:?}", pos);

    // Color
    let color = RGBAColor::new(0.2, 0.5, 0.8, 1.0);
    println!("Color: {:?}", color);

    // Pixel
    let pixel = Pixel {
        location: pos,
        depth: 0.75,
        color,
    };
    println!("Pixel: {:?}", pixel);

    println!("\n--- Testing Physics ---");

    // Velocity
    let mut vel = Velocity::new(Vec3::new(1.0, 0.0, 0.0));
    println!("Velocity: {:?}, speed: {}", vel.vector, vel.speed());

    // Acceleration
    let accel = Acceleration::new(Vec3::new(0.0, -9.8, 0.0));
    println!("Acceleration: {:?}, magnitude: {}", accel.vector, accel.magnitude());

    // Apply acceleration to velocity
    let dt = 1.0; // 1 second timestep
    vel.apply_acceleration(&accel.vector, dt);
    println!("Velocity after {}s: {:?}, speed: {}", dt, vel.vector, vel.speed());

    // Force
    let force = Force::new(Vec3::new(10.0, 0.0, 0.0), Some(pos));
    println!("Force: {:?}, magnitude: {}", force.vector, force.magnitude());

    // Convert force to acceleration
    let mass = 2.0;
    let lin_accel = force.to_acceleration(mass);
    println!("Linear acceleration from force on mass {}: {:?}", mass, lin_accel);
}