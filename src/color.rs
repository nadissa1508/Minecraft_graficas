use std::ops::{Add, Mul};
use crate::utils::{Vec3, clamp};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn black() -> Self { Self::new(0.0, 0.0, 0.0) }
    pub fn white() -> Self { Self::new(1.0, 1.0, 1.0) }
    pub fn red() -> Self { Self::new(1.0, 0.0, 0.0) }
    pub fn green() -> Self { Self::new(0.0, 1.0, 0.0) }
    pub fn blue() -> Self { Self::new(0.0, 0.0, 1.0) }

    pub fn from_vec3(v: Vec3) -> Self {
        Self::new(v.x, v.y, v.z)
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.r, self.g, self.b)
    }

    pub fn to_raylib(&self) -> raylib::prelude::Color {
        raylib::prelude::Color::new(
            (clamp(self.r, 0.0, 1.0) * 255.0) as u8,
            (clamp(self.g, 0.0, 1.0) * 255.0) as u8,
            (clamp(self.b, 0.0, 1.0) * 255.0) as u8,
            255,
        )
    }

    pub fn from_u8(r: u8, g: u8, b: u8) -> Self {
        Self::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
        )
    }

    pub fn clamp(&self) -> Self {
        Self::new(
            clamp(self.r, 0.0, 1.0),
            clamp(self.g, 0.0, 1.0),
            clamp(self.b, 0.0, 1.0),
        )
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, s: f32) -> Color {
        Color::new(self.r * s, self.g * s, self.b * s)
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        Color::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}
