use crate::scene::Scene;
use crate::camera::Camera;
use crate::ray::Ray;
use crate::color::Color;

const MAX_DEPTH: i32 = 5;

pub fn render_scene(
    scene: &Scene,
    camera: &Camera,
    buffer: &mut [raylib::prelude::Color],
    width: i32,
    height: i32,
    render_scale: i32,
    use_threading: bool,
) {
    let scaled_width = width / render_scale;
    let scaled_height = height / render_scale;

    if use_threading {
        render_threaded(scene, camera, buffer, width, height, scaled_width, scaled_height, render_scale);
    } else {
        render_single_threaded(scene, camera, buffer, width, height, scaled_width, scaled_height, render_scale);
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
) {
    for sy in 0..scaled_height {
        for sx in 0..scaled_width {
            let u = sx as f32 / scaled_width as f32;
            let v = sy as f32 / scaled_height as f32;

            let ray = camera.get_ray(u, v);
            let color = trace_ray(&ray, scene, 0);

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
                    let color = trace_ray(&ray, &scene, 0);

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

fn trace_ray(ray: &Ray, scene: &Scene, depth: i32) -> Color {
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

        // Ambient lighting (increased so all surfaces are visible)
        let ambient = Color::new(0.4, 0.4, 0.45);

        // Diffuse lighting from sun
        let light_dir = -scene.sun.direction;
        let diffuse_strength = normal.dot(&light_dir).max(0.0);

        // Shadow check
        let shadow_ray = Ray::new(hit_point + normal * 0.001, light_dir);
        let in_shadow = scene.intersect(&shadow_ray).is_some();

        let diffuse = if in_shadow {
            Color::black()
        } else {
            scene.sun.color * (diffuse_strength * scene.sun.intensity)
        };

        let mut color = (ambient + diffuse) * surface_color;

        // Reflection
        if material.reflectivity > 0.0 {
            let reflect_dir = ray.direction.reflect(&normal);
            let reflect_ray = Ray::new(hit_point + normal * 0.001, reflect_dir);
            let reflect_color = trace_ray(&reflect_ray, scene, depth + 1);
            color = color * (1.0 - material.reflectivity) + reflect_color * material.reflectivity;
        }

        // Refraction
        if material.transparency > 0.0 {
            let eta = 1.0 / material.refractive_index;
            if let Some(refract_dir) = ray.direction.refract(&normal, eta) {
                let refract_ray = Ray::new(hit_point - normal * 0.001, refract_dir);
                let refract_color = trace_ray(&refract_ray, scene, depth + 1);
                color = color * (1.0 - material.transparency) + refract_color * material.transparency;
            }
        }

        color.clamp()
    } else {
        // Sky
        scene.skybox.sample(ray, 0.0)
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
            day_color_top: self.day_color_top,
            day_color_horizon: self.day_color_horizon,
            night_color_top: self.night_color_top,
            night_color_horizon: self.night_color_horizon,
        }
    }
}
