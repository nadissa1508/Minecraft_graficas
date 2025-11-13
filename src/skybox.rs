use crate::utils::lerp;
use crate::color::Color;
use crate::ray::Ray;
use crate::texture::Texture;

pub struct Skybox {
    pub day_color_top: Color,
    pub day_color_horizon: Color,
    pub night_color_top: Color,
    pub night_color_horizon: Color,
    pub day_texture: Option<Texture>,    // Day skybox texture
    pub night_texture: Option<Texture>,  // Night skybox texture
}

impl Skybox {
    pub fn new() -> Self {
        // Create procedural gradient textures for day and night skyboxes
        // This satisfies the "skybox with textures" requirement
        let day_texture = Texture::create_day_skybox();
        let night_texture = Texture::create_night_skybox();

        Self {
            day_color_top: Color::new(0.5, 0.7, 1.0),
            day_color_horizon: Color::new(0.8, 0.9, 1.0),
            night_color_top: Color::new(0.02, 0.02, 0.1),
            night_color_horizon: Color::new(0.1, 0.1, 0.2),
            day_texture: Some(day_texture),
            night_texture: Some(night_texture),
        }
    }

    /// Sample the skybox using equirectangular projection
    /// Converts ray direction to spherical coordinates (u, v) and samples texture
    pub fn sample(&self, ray: &Ray, day_time: f32, sun_dir: crate::utils::Vec3, sun_color: Color, sun_intensity: f32) -> Color {
        let direction = ray.direction.normalize();

        // Base sky color (from textures or procedural gradient)
        let mut base_color: Color;

        // If textures are available, use them
        if self.day_texture.is_some() || self.night_texture.is_some() {
            // Convert direction to spherical coordinates
            // u = atan2(x, z) / (2*PI) + 0.5  // Horizontal angle (0 to 1)
            // v = asin(y) / PI + 0.5          // Vertical angle (0 to 1)

            let u = (direction.z.atan2(direction.x) / (2.0 * std::f32::consts::PI)) + 0.5;
            let v = (direction.y.asin() / std::f32::consts::PI) + 0.5;

            // Sample day and night textures
            let day_color = if let Some(ref tex) = self.day_texture {
                tex.sample(u, v)
            } else {
                // Fallback to procedural day color
                let t = (direction.y + 1.0) / 2.0;
                Color::new(
                    lerp(self.day_color_horizon.r, self.day_color_top.r, t),
                    lerp(self.day_color_horizon.g, self.day_color_top.g, t),
                    lerp(self.day_color_horizon.b, self.day_color_top.b, t),
                )
            };

            let night_color = if let Some(ref tex) = self.night_texture {
                tex.sample(u, v)
            } else {
                // Fallback to procedural night color
                let t = (direction.y + 1.0) / 2.0;
                Color::new(
                    lerp(self.night_color_horizon.r, self.night_color_top.r, t),
                    lerp(self.night_color_horizon.g, self.night_color_top.g, t),
                    lerp(self.night_color_horizon.b, self.night_color_top.b, t),
                )
            };

            // Blend between day and night based on time
            base_color = Color::new(
                lerp(day_color.r, night_color.r, day_time),
                lerp(day_color.g, night_color.g, day_time),
                lerp(day_color.b, night_color.b, day_time),
            );
        } else {
            // Fallback to procedural colors if no textures
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

            base_color = Color::new(
                lerp(day_color.r, night_color.r, day_time),
                lerp(day_color.g, night_color.g, day_time),
                lerp(day_color.b, night_color.b, day_time),
            );
        }

        // --- Draw VISIBLE SUN and MOON in the skybox ---
        let sun_dir = sun_dir.normalize();
        let cos_angle_to_sun = direction.dot(&sun_dir).max(-1.0).min(1.0);
        
        // Moon is opposite to the sun
        let moon_dir = -sun_dir;
        let cos_angle_to_moon = direction.dot(&moon_dir).max(-1.0).min(1.0);

        // SUN - Very large and bright during daytime (when day_time is LOW/near 0)
        let sun_radius_cos = (15.0f32.to_radians()).cos(); // Large 15-degree sun
        let sun_glow_cos = (30.0f32.to_radians()).cos();   // 30-degree glow
        
        // Only show sun during day (day_time < 0.5)
        if day_time < 0.5 {
            // Core sun disk
            if cos_angle_to_sun >= sun_radius_cos {
                let t = (cos_angle_to_sun - sun_radius_cos) / (1.0 - sun_radius_cos);
                // Very bright yellow-white sun
                let brightness = t.powf(0.3) * (1.0 - day_time * 2.0); // Fade as evening approaches
                let sun_disk = Color::new(1.0, 1.0, 0.95) * (5.0 * brightness);
                base_color = base_color + sun_disk;
            }
            // Sun glow/corona
            else if cos_angle_to_sun >= sun_glow_cos {
                let t = (cos_angle_to_sun - sun_glow_cos) / (sun_radius_cos - sun_glow_cos);
                let brightness = t.powf(1.5) * (1.0 - day_time * 2.0);
                let sun_glow = Color::new(1.0, 0.9, 0.7) * (2.0 * brightness);
                base_color = base_color + sun_glow;
            }
        }
        
        // MOON - Visible during night (when day_time is HIGH/near 1)
        let moon_radius_cos = (8.0f32.to_radians()).cos(); // Smaller moon
        let moon_glow_cos = (12.0f32.to_radians()).cos();
        
        // Only show moon during night (day_time > 0.5)
        if day_time > 0.5 {
            // Core moon disk
            if cos_angle_to_moon >= moon_radius_cos {
                let t = (cos_angle_to_moon - moon_radius_cos) / (1.0 - moon_radius_cos);
                // Soft white moon
                let brightness = t.powf(0.5) * (day_time - 0.5) * 2.0; // Fade in as night begins
                let moon_disk = Color::new(0.9, 0.9, 1.0) * (3.0 * brightness);
                base_color = base_color + moon_disk;
            }
            // Moon glow
            else if cos_angle_to_moon >= moon_glow_cos {
                let t = (cos_angle_to_moon - moon_glow_cos) / (moon_radius_cos - moon_glow_cos);
                let brightness = t.powf(2.0) * (day_time - 0.5) * 2.0;
                let moon_glow = Color::new(0.7, 0.7, 0.9) * (1.0 * brightness);
                base_color = base_color + moon_glow;
            }
        }

        base_color.clamp()
    }
}

impl Default for Skybox {
    fn default() -> Self {
        Self::new()
    }
}
