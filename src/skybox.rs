use crate::utils::lerp;
use crate::color::Color;
use crate::ray::Ray;

pub struct Skybox {
    pub day_color_top: Color,
    pub day_color_horizon: Color,
    pub night_color_top: Color,
    pub night_color_horizon: Color,
}

impl Skybox {
    pub fn new() -> Self {
        Self {
            day_color_top: Color::new(0.5, 0.7, 1.0),
            day_color_horizon: Color::new(0.8, 0.9, 1.0),
            night_color_top: Color::new(0.02, 0.02, 0.1),
            night_color_horizon: Color::new(0.1, 0.1, 0.2),
        }
    }

    // TODO: Implement cubemap texture loading
    pub fn sample(&self, ray: &Ray, day_time: f32) -> Color {
        let direction = ray.direction.normalize();
        let t = (direction.y + 1.0) / 2.0;

        let day_color = Color::new(
            lerp(self.day_color_horizon.r, self.day_color_top.r, t),
            lerp(self.day_color_horizon.g, self.day_color_top.g, t),
            lerp(self.day_color_horizon.b, self.day_color_top.b, t),
        );

        let night_color = Color::new(
            lerp(self.night_color_horizon.r, self.night_color_top.r, t),
            lerp(self.night_color_horizon.g, self.night_color_top.g, t),
            lerp(self.night_color_horizon.b, self.night_color_top.b, t),
        );

        // Blend between day and night
        Color::new(
            lerp(day_color.r, night_color.r, day_time),
            lerp(day_color.g, night_color.g, day_time),
            lerp(day_color.b, night_color.b, day_time),
        )
    }
}

impl Default for Skybox {
    fn default() -> Self {
        Self::new()
    }
}
