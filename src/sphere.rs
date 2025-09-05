//sphere.rs
use nalgebra_glm::Vec3;
use crate::{ray_intersect::{RayIntersect, Intersect, Material}, color::Color};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let l = self.center - ray_origin;
        let tca = l.dot(ray_direction);
        let d2 = l.dot(&l) - tca * tca;

        if d2 > self.radius * self.radius {
            return Intersect::empty();
        }

        let thc = (self.radius * self.radius - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;

        let distance = if t0 < 0.0 { t1 } else { t0 };

        if distance < 0.0 {
            return Intersect::empty();
        }

        Intersect::new(distance, self.material)
    }
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, color: Color) -> Self {
        Sphere {
            center,
            radius,
            material: Material { diffuse: color },
        }
    }
}
