use crate::utils::Vec3;
use crate::color::Color;

pub struct DirectionalLight {
    pub direction: Vec3,
    pub color: Color,
    pub intensity: f32,
}

impl DirectionalLight {
    pub fn new(direction: Vec3, color: Color, intensity: f32) -> Self {
        Self {
            direction: direction.normalize(),
            color,
            intensity,
        }
    }

    pub fn sun(direction: Vec3, intensity: f32) -> Self {
        Self::new(direction, Color::new(1.0, 0.95, 0.9), intensity)
    }
}

pub struct PointLight {
    pub position: Vec3,
    pub color: Color,
    pub intensity: f32,
}

impl PointLight {
    pub fn new(position: Vec3, color: Color, intensity: f32) -> Self {
        Self {
            position,
            color,
            intensity,
        }
    }
}
