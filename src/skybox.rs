use crate::color::Color;
use crate::ray::Ray;
use crate::texture::Texture;

pub struct Skybox {
    // Cubemap textures (6 faces)
    pub right: Texture,   // +X
    pub left: Texture,    // -X
    pub top: Texture,     // +Y
    pub bottom: Texture,  // -Y
    pub front: Texture,   // +Z
    pub back: Texture,    // -Z
}

impl Skybox {
    pub fn new() -> Self {
        // Load the 6 cubemap face textures from assets/skybox/
        Self {
            right: Texture::load("assets/skybox/side.jpeg"),
            left: Texture::load("assets/skybox/side.jpeg"),
            top: Texture::load("assets/skybox/top.jpeg"),
            bottom: Texture::load("assets/skybox/bottom.jpg"),
            front: Texture::load("assets/skybox/side.jpeg"),
            back: Texture::load("assets/skybox/side.jpeg"),
        }
    }

    /// Sample the skybox cubemap based on ray direction
    /// This uses the standard cubemap sampling algorithm
    pub fn sample(&self, ray: &Ray, day_time: f32, sun_dir: crate::utils::Vec3, _sun_color: Color, _sun_intensity: f32) -> Color {
        let direction = ray.direction.normalize();
        
        // Determine which cube face to sample based on the largest component
        let abs_x = direction.x.abs();
        let abs_y = direction.y.abs();
        let abs_z = direction.z.abs();
        
        let (u, v, texture) = if abs_x >= abs_y && abs_x >= abs_z {
            // X is dominant
            if direction.x > 0.0 {
                // Right face (+X)
                let u = (-direction.z / abs_x + 1.0) * 0.5;
                let v = (-direction.y / abs_x + 1.0) * 0.5;
                (u, v, &self.right)
            } else {
                // Left face (-X)
                let u = (direction.z / abs_x + 1.0) * 0.5;
                let v = (-direction.y / abs_x + 1.0) * 0.5;
                (u, v, &self.left)
            }
        } else if abs_y >= abs_x && abs_y >= abs_z {
            // Y is dominant
            if direction.y > 0.0 {
                // Top face (+Y)
                let u = (direction.x / abs_y + 1.0) * 0.5;
                let v = (direction.z / abs_y + 1.0) * 0.5;
                (u, v, &self.top)
            } else {
                // Bottom face (-Y)
                let u = (direction.x / abs_y + 1.0) * 0.5;
                let v = (-direction.z / abs_y + 1.0) * 0.5;
                (u, v, &self.bottom)
            }
        } else {
            // Z is dominant
            if direction.z > 0.0 {
                // Front face (+Z)
                let u = (direction.x / abs_z + 1.0) * 0.5;
                let v = (-direction.y / abs_z + 1.0) * 0.5;
                (u, v, &self.front)
            } else {
                // Back face (-Z)
                let u = (-direction.x / abs_z + 1.0) * 0.5;
                let v = (-direction.y / abs_z + 1.0) * 0.5;
                (u, v, &self.back)
            }
        };
        
        // Sample the texture
        let mut base_color = texture.sample(u, v);
        
        // Apply day/night tinting
        // During day (day_time = 0), keep original colors
        // During night (day_time = 1), darken and add blue tint
        if day_time > 0.0 {
            let night_tint = Color::new(0.1, 0.1, 0.2) * day_time;
            let darken = 1.0 - (day_time * 0.8); // Darken by up to 80% at full night
            base_color = base_color * darken + night_tint;
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
