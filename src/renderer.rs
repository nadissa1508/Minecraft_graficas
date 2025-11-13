use crate::scene::Scene;
use crate::camera::Camera;
use crate::ray::Ray;
use crate::color::Color;

const MAX_DEPTH: i32 = 8;  // Increased from 5 to 8 for better water transparency/reflection

pub fn render_scene(
    scene: &Scene,
    camera: &Camera,
    buffer: &mut [raylib::prelude::Color],
    width: i32,
    height: i32,
    render_scale: i32,
    use_threading: bool,
    day_time: f32,
) {
    let scaled_width = width / render_scale;
    let scaled_height = height / render_scale;

    if use_threading {
        render_threaded(scene, camera, buffer, width, height, scaled_width, scaled_height, render_scale, day_time);
    } else {
        render_single_threaded(scene, camera, buffer, width, height, scaled_width, scaled_height, render_scale, day_time);
    }
}

fn render_single_threaded(
    scene: &Scene,
    camera: &Camera,
    buffer: &mut [raylib::prelude::Color],
    width: i32,
    height: i32,
    scaled_width: i32,
    scaled_height: i32,
    render_scale: i32,
    day_time: f32,
) {
    for sy in 0..scaled_height {
        for sx in 0..scaled_width {
            let u = sx as f32 / scaled_width as f32;
            let v = sy as f32 / scaled_height as f32;

            let ray = camera.get_ray(u, v);
            let color = trace_ray(&ray, scene, 0, day_time);

            // Fill the scaled pixels
            for dy in 0..render_scale {
                for dx in 0..render_scale {
                    let x = sx * render_scale + dx;
                    let y = sy * render_scale + dy;
                    if x < width && y < height {
                        let idx = (y * width + x) as usize;
                        buffer[idx] = color.to_raylib();
                    }
                }
            }
        }
    }
}

fn render_threaded(
    scene: &Scene,
    camera: &Camera,
    buffer: &mut [raylib::prelude::Color],
    width: i32,
    height: i32,
    scaled_width: i32,
    scaled_height: i32,
    render_scale: i32,
    day_time: f32,
) {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let num_threads = 4;
    let buffer = Arc::new(Mutex::new(buffer));
    let scene = Arc::new(scene.clone());
    let camera = Arc::new(*camera);

    let rows_per_thread = (scaled_height + num_threads - 1) / num_threads;

    let mut handles = vec![];

    for thread_id in 0..num_threads {
        let scene = Arc::clone(&scene);
        let camera = Arc::clone(&camera);

        let start_row = thread_id * rows_per_thread;
        let end_row = ((thread_id + 1) * rows_per_thread).min(scaled_height);

        let handle = thread::spawn(move || {
            let mut local_pixels = vec![];

            for sy in start_row..end_row {
                for sx in 0..scaled_width {
                    let u = sx as f32 / scaled_width as f32;
                    let v = sy as f32 / scaled_height as f32;

                    let ray = camera.get_ray(u, v);
                    let color = trace_ray(&ray, &scene, 0, day_time);

                    for dy in 0..render_scale {
                        for dx in 0..render_scale {
                            let x = sx * render_scale + dx;
                            let y = sy * render_scale + dy;
                            if x < width && y < height {
                                let idx = (y * width + x) as usize;
                                local_pixels.push((idx, color.to_raylib()));
                            }
                        }
                    }
                }
            }

            local_pixels
        });

        handles.push(handle);
    }

    for handle in handles {
        if let Ok(pixels) = handle.join() {
            let mut buffer = buffer.lock().unwrap();
            for (idx, color) in pixels {
                buffer[idx] = color;
            }
        }
    }
}

fn trace_ray(ray: &Ray, scene: &Scene, depth: i32, day_time: f32) -> Color {
    if depth >= MAX_DEPTH {
        return Color::black();
    }

    if let Some(intersection) = scene.intersect(ray) {
        let material = &intersection.material;
        let normal = intersection.normal;
        let hit_point = intersection.position;

        // Get surface color
        let surface_color = material.get_color(intersection.u, intersection.v);

        // Emissive
        if material.emissive.r > 0.0 || material.emissive.g > 0.0 || material.emissive.b > 0.0 {
            return material.emissive;
        }

        // Ambient lighting - varies with day/night cycle
        // Day (day_time=0.0): Bright ambient light
        // Night (day_time=1.0): Very dark ambient light
        let day_ambient = Color::new(0.45, 0.45, 0.52);
        let night_ambient = Color::new(0.05, 0.05, 0.08); // Very dark at night
        let ambient = day_ambient * (1.0 - day_time) + night_ambient * day_time;

        // View direction for specular calculations
        let view_dir = -ray.direction;

        // Sun/moon intensity varies with day/night
        // During day (day_time=0.0): Full sun intensity
        // During night (day_time=1.0): Very weak moonlight
        let celestial_intensity = scene.sun.intensity * (1.0 - day_time * 0.95); // Reduce to 5% at night

        // Diffuse lighting from sun
        let light_dir = -scene.sun.direction;
        let diffuse_strength = normal.dot(&light_dir).max(0.0);

        // Shadow check
        let shadow_ray = Ray::new(hit_point + normal * 0.001, light_dir);
        let in_shadow = scene.intersect(&shadow_ray).is_some();

        let diffuse = if in_shadow {
            Color::black()
        } else {
            scene.sun.color * (diffuse_strength * celestial_intensity)
        };

        // Specular lighting from sun (Blinn-Phong)
        let mut specular = Color::black();
        if !in_shadow && material.specular > 0.0 && diffuse_strength > 0.0 {
            let halfway = (light_dir + view_dir).normalize();
            let spec_strength = normal.dot(&halfway).max(0.0).powf(material.shininess);
            specular = scene.sun.color * (material.specular * spec_strength * celestial_intensity);
        }

        // Add point light contributions (diffuse + specular)
        let mut point_light_contribution = Color::black();
        let mut point_light_specular = Color::black();
        for point_light in &scene.point_lights {
            let (light_direction, light_color) = point_light.illuminate(&hit_point);

            // Skip if light is too far or has no contribution
            if light_color.r <= 0.0 && light_color.g <= 0.0 && light_color.b <= 0.0 {
                continue;
            }

            // Calculate diffuse strength for this point light
            let point_diffuse_strength = normal.dot(&light_direction).max(0.0);

            // Shadow check for this point light
            let point_shadow_ray = Ray::new(hit_point + normal * 0.001, light_direction);
            let point_in_shadow = if let Some(shadow_hit) = scene.intersect(&point_shadow_ray) {
                // Check if the shadow hit is closer than the light source
                let light_distance = (point_light.position - hit_point).length();
                shadow_hit.t < light_distance
            } else {
                false
            };

            if !point_in_shadow && point_diffuse_strength > 0.0 {
                // Diffuse contribution
                point_light_contribution = point_light_contribution + light_color * point_diffuse_strength;

                // Specular contribution (Blinn-Phong)
                if material.specular > 0.0 {
                    let halfway = (light_direction + view_dir).normalize();
                    let spec_strength = normal.dot(&halfway).max(0.0).powf(material.shininess);
                    point_light_specular = point_light_specular + light_color * (material.specular * spec_strength);
                }
            }
        }

        let mut color = (ambient + diffuse + point_light_contribution) * surface_color + specular + point_light_specular;

        // Calculate Fresnel effect for more realistic reflections (especially for water)
        let cos_theta = view_dir.dot(&normal).abs().max(0.0).min(1.0);
        
        // Schlick's approximation for Fresnel reflectance
        let r0 = if material.refractive_index > 1.0 {
            ((1.0 - material.refractive_index) / (1.0 + material.refractive_index)).powi(2)
        } else {
            0.04 // Default for non-refractive materials
        };
        let fresnel = r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5);

        // Reflection (enhanced with Fresnel for transparent materials)
        if material.reflectivity > 0.0 || material.transparency > 0.0 {
            let reflect_dir = ray.direction.reflect(&normal);
            let reflect_ray = Ray::new(hit_point + normal * 0.001, reflect_dir);
            let reflect_color = trace_ray(&reflect_ray, scene, depth + 1, day_time);

            // Use Fresnel for transparent materials, otherwise use base reflectivity
            let effective_reflectivity = if material.transparency > 0.0 {
                fresnel.max(material.reflectivity)
            } else {
                material.reflectivity
            };

            color = color * (1.0 - effective_reflectivity) + reflect_color * effective_reflectivity;
        }

        // Refraction
        if material.transparency > 0.0 {
            let eta = 1.0 / material.refractive_index;
            if let Some(refract_dir) = ray.direction.refract(&normal, eta) {
                let refract_ray = Ray::new(hit_point - normal * 0.001, refract_dir);
                let refract_color = trace_ray(&refract_ray, scene, depth + 1, day_time);

                // Blend refraction with existing color (accounting for Fresnel in reflection above)
                let refract_amount = material.transparency * (1.0 - fresnel);
                color = color * (1.0 - refract_amount) + refract_color * refract_amount;
            }
        }

        color.clamp()
    } else {
        // Sky - use actual day_time for skybox texture blending
        // Pass sun parameters so the skybox can render a visible sun disk
        scene.skybox.sample(ray, day_time, -scene.sun.direction, scene.sun.color, scene.sun.intensity)
    }
}

// Copy trait for Camera
impl Copy for Camera {}
impl Clone for Camera {
    fn clone(&self) -> Self {
        *self
    }
}

// Clone trait for Scene (needed for threading)
impl Clone for Scene {
    fn clone(&self) -> Self {
        Self {
            cubes: self.cubes.iter().map(|c| c.clone()).collect(),
            meshes: self.meshes.iter().map(|m| m.clone()).collect(),
            sun: self.sun.clone(),
            point_lights: self.point_lights.iter().map(|l| l.clone()).collect(),
            skybox: self.skybox.clone(),
        }
    }
}

impl Clone for crate::cube::Cube {
    fn clone(&self) -> Self {
        Self {
            position: self.position,
            size: self.size,
            material: self.material.clone(),
            top_material: self.top_material.clone(),
            side_material: self.side_material.clone(),
            bottom_material: self.bottom_material.clone(),
        }
    }
}

impl Clone for crate::obj_loader::Mesh {
    fn clone(&self) -> Self {
        Self {
            triangles: self.triangles.iter().map(|t| t.clone()).collect(),
            position: self.position,
            scale: self.scale,
            material: self.material.clone(),
        }
    }
}

impl Clone for crate::obj_loader::Triangle {
    fn clone(&self) -> Self {
        Self {
            v0: self.v0,
            v1: self.v1,
            v2: self.v2,
            normal: self.normal,
        }
    }
}

impl Clone for crate::light::DirectionalLight {
    fn clone(&self) -> Self {
        Self {
            direction: self.direction,
            color: self.color,
            intensity: self.intensity,
        }
    }
}

impl Clone for crate::skybox::Skybox {
    fn clone(&self) -> Self {
        Self {
            right_day: self.right_day.clone(),
            left_day: self.left_day.clone(),
            top_day: self.top_day.clone(),
            bottom_day: self.bottom_day.clone(),
            front_day: self.front_day.clone(),
            back_day: self.back_day.clone(),
            right_night: self.right_night.clone(),
            left_night: self.left_night.clone(),
            top_night: self.top_night.clone(),
            bottom_night: self.bottom_night.clone(),
            front_night: self.front_night.clone(),
            back_night: self.back_night.clone(),
        }
    }
}
