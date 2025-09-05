// ray_tracing.rs
use crate::ray_intersect::RayIntersect;
use crate::{
    color::Color, cube::Cube, framebuffer::Framebuffer, light::Light, ray_intersect::Intersect,
    sphere::Sphere,
};
use nalgebra_glm::{Vec3, dot, normalize};

pub enum Object {
    Sphere(Sphere),
    Cube(Cube),
}

impl RayIntersect for Object {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        match self {
            Object::Sphere(sphere) => sphere.ray_intersect(ray_origin, ray_direction),
            Object::Cube(cube) => cube.ray_intersect(ray_origin, ray_direction),
        }
    }
}

impl Object {
    pub fn get_normal(&self, hit_point: &Vec3) -> Vec3 {
        match self {
            Object::Sphere(sphere) => normalize(&(hit_point - sphere.center)),
            Object::Cube(cube) => cube.get_normal(hit_point),
        }
    }
}

fn calculate_diffuse_lighting(
    hit_point: &Vec3,
    normal: &Vec3,
    light: &Light,
    base_color: &Color,
) -> Color {
    let light_dir = normalize(&(light.position - hit_point));
    let diffuse_intensity = dot(normal, &light_dir).max(0.0);

    let final_intensity = diffuse_intensity * light.intensity;

    Color::new(
        ((base_color.r as f32 * light.color.r as f32 * final_intensity) / 255.0).min(255.0) as u8,
        ((base_color.g as f32 * light.color.g as f32 * final_intensity) / 255.0).min(255.0) as u8,
        ((base_color.b as f32 * light.color.b as f32 * final_intensity) / 255.0).min(255.0) as u8,
    )
}

pub fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[Object],
    lights: &[Light],
) -> Color {
    let mut closest = Intersect::empty();
    let mut zbuffer = f32::INFINITY;
    let mut closest_object: Option<&Object> = None;

    for obj in objects {
        let hit = obj.ray_intersect(ray_origin, ray_direction);
        if hit.is_intersecting && hit.distance < zbuffer {
            zbuffer = hit.distance;
            closest = hit;
            closest_object = Some(obj);
        }
    }

    if closest.is_intersecting {
        if let Some(obj) = closest_object {
            let hit_point = ray_origin + ray_direction * closest.distance;
            let normal = obj.get_normal(&hit_point);

            let mut final_color = Color::new(0, 0, 0);

            // Luz ambiental
            let ambient_strength = 0.1;
            let ambient_r = (closest.material.diffuse.r as f32 * ambient_strength) as u8;
            let ambient_g = (closest.material.diffuse.g as f32 * ambient_strength) as u8;
            let ambient_b = (closest.material.diffuse.b as f32 * ambient_strength) as u8;

            final_color.r = ambient_r;
            final_color.g = ambient_g;
            final_color.b = ambient_b;

            // Agregar contribución de cada luz
            for light in lights {
                let diffuse_color = calculate_diffuse_lighting(
                    &hit_point,
                    &normal,
                    light,
                    &closest.material.diffuse,
                );

                final_color.r = (final_color.r as u16 + diffuse_color.r as u16).min(255) as u8;
                final_color.g = (final_color.g as u16 + diffuse_color.g as u16).min(255) as u8;
                final_color.b = (final_color.b as u16 + diffuse_color.b as u16).min(255) as u8;
            }

            return final_color;
        }
    }

    Color::new(135, 206, 235) // color del cielo
}

// En ray_tracing.rs, actualiza la función render:
pub fn render(framebuffer: &mut Framebuffer, objects: &[Object], lights: &[Light]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    // Posición de la cámara (para vista isométrica)
    let camera_pos = Vec3::new(4.0, 4.0, 6.0);
    let target = Vec3::new(0.0, 0.0, -5.0);

    let fov = std::f32::consts::PI / 3.0; // 60 grados
    let scale = (fov * 0.5).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            framebuffer.point(x, y, framebuffer.background_color);
            let screen_x = (2.0 * (x as f32 + 0.5) / width - 1.0) * aspect_ratio * scale;
            let screen_y = (1.0 - 2.0 * (y as f32 + 0.5) / height) * scale;

            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));
            let color = cast_ray(&camera_pos, &ray_direction, objects, lights);

            framebuffer.point(x, y, color);
        }
    }
}
