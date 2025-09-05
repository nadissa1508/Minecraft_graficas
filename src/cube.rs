//cube.rs
use nalgebra_glm::Vec3;
use crate::{ray_intersect::{RayIntersect, Intersect, Material}, color::Color};

pub struct Cube {
    pub min: Vec3,
    pub max: Vec3,
    pub material: Material,
}

impl Cube {
    pub fn new(center: Vec3, size: f32, color: Color) -> Self {
        let half_size = size / 2.0;
        Cube {
            min: Vec3::new(
                center.x - half_size,
                center.y - half_size,
                center.z - half_size,
            ),
            max: Vec3::new(
                center.x + half_size,
                center.y + half_size,
                center.z + half_size,
            ),
            material: Material { diffuse: color },
        }
    }

    pub fn new_with_dimensions(center: Vec3, dimensions: Vec3, color: Color) -> Self {
        let half_dims = dimensions * 0.5;
        Cube {
            min: center - half_dims,
            max: center + half_dims,
            material: Material { diffuse: color },
        }
    }

    pub fn get_normal(&self, hit_point: &Vec3) -> Vec3 {
        let center = (self.min + self.max) * 0.5;
        let p = *hit_point - center;
        let d = (self.max - self.min) * 0.5;
        
        let bias = 1.0001;
        
        if (p.x / d.x).abs() > bias {
            return Vec3::new(p.x.signum(), 0.0, 0.0);
        }
        if (p.y / d.y).abs() > bias {
            return Vec3::new(0.0, p.y.signum(), 0.0);
        }
        if (p.z / d.z).abs() > bias {
            return Vec3::new(0.0, 0.0, p.z.signum());
        }
        
        if p.x.abs() > p.y.abs() && p.x.abs() > p.z.abs() {
            Vec3::new(p.x.signum(), 0.0, 0.0)
        } else if p.y.abs() > p.z.abs() {
            Vec3::new(0.0, p.y.signum(), 0.0)
        } else {
            Vec3::new(0.0, 0.0, p.z.signum())
        }
    }
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let inv_dir = Vec3::new(
            if ray_direction.x != 0.0 { 1.0 / ray_direction.x } else { f32::INFINITY },
            if ray_direction.y != 0.0 { 1.0 / ray_direction.y } else { f32::INFINITY },
            if ray_direction.z != 0.0 { 1.0 / ray_direction.z } else { f32::INFINITY },
        );

        let t1 = (self.min.x - ray_origin.x) * inv_dir.x;
        let t2 = (self.max.x - ray_origin.x) * inv_dir.x;
        let t3 = (self.min.y - ray_origin.y) * inv_dir.y;
        let t4 = (self.max.y - ray_origin.y) * inv_dir.y;
        let t5 = (self.min.z - ray_origin.z) * inv_dir.z;
        let t6 = (self.max.z - ray_origin.z) * inv_dir.z;

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        if tmax < 0.0 || tmin > tmax {
            return Intersect::empty();
        }

        let distance = if tmin < 0.0 { tmax } else { tmin };

        if distance < 0.0 {
            return Intersect::empty();
        }

        Intersect::new(distance, self.material)
    }
}