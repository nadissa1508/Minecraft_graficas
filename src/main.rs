// main.rs
mod framebuffer;
mod ray_tracing;
mod ray_intersect;
mod sphere;
mod cube;
mod light;
mod color;

use nalgebra_glm::Vec3;
use raylib::prelude::*;
use framebuffer::Framebuffer;
use cube::Cube;
use light::Light;
use color::Color;
use ray_tracing::{render, Object};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Cubo Rosado con Luz Difusa")
        .build();

    // Fondo azul oscuro
    let mut framebuffer = Framebuffer::new(800, 600, Color::new(0, 0, 139));

    let objects = vec![
        // Un solo cubo rosado en el centro
        Object::Cube(Cube::new(
            Vec3::new(0.0, 0.0, -5.0), 
            2.5, 
            Color::new(255, 105, 180) // Rosa claro
        )),
    ];

    let lights = vec![
        // Una sola luz principal
        Light::new(
            Vec3::new(-2.0, 2.0, -2.0), 
            Color::new(255, 255, 255), 
            1.0
        ),
    ];

    render(&mut framebuffer, &objects, &lights);

    while !rl.window_should_close() {
        framebuffer.swap_buffers(&mut rl, &thread);
    }
}