use crate::color::Color;
use crate::utils::Vec3;

#[derive(Clone)]
pub struct PointLight {
    pub position: Vec3,
    pub color: Color,
    pub intensity: f32,
    pub radius: f32, // Maximum distance the light can reach
}

impl PointLight {
    pub fn new(position: Vec3, color: Color, intensity: f32, radius: f32) -> Self {
        Self {
            position,
            color,
            intensity,
            radius,
        }
    }

    /// Calculate the light contribution at a given point
    /// Returns (light_direction, light_color_with_attenuation)
    pub fn illuminate(&self, point: &Vec3) -> (Vec3, Color) {
        let light_vec = self.position - *point;
        let distance = light_vec.length();

        // No illumination beyond radius
        if distance > self.radius {
            return (Vec3::new(0.0, 0.0, 0.0), Color::black());
        }

        let light_dir = light_vec.normalize();

        // Quadratic attenuation: 1 / (1 + d^2)
        // This makes light fall off realistically with distance
        let attenuation = 1.0 / (1.0 + distance * distance * 0.5);

        let attenuated_color = self.color * (self.intensity * attenuation);

        (light_dir, attenuated_color)
    }
}
