use crate::utils::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use crate::intersection::Intersection;

pub struct Cube {
    pub position: Vec3,
    pub size: f32,
    pub material: Material,
    pub top_material: Option<Material>,
    pub side_material: Option<Material>,
    pub bottom_material: Option<Material>,
}

impl Cube {
    pub fn new(position: Vec3, size: f32, material: Material) -> Self {
        Self {
            position,
            size,
            material,
            top_material: None,
            side_material: None,
            bottom_material: None,
        }
    }

    // Create a cube with different materials for top, sides, and bottom
    pub fn new_multi_texture(
        position: Vec3,
        size: f32,
        top: Material,
        sides: Material,
        bottom: Material,
    ) -> Self {
        Self {
            position,
            size,
            material: sides.clone(),
            top_material: Some(top),
            side_material: Some(sides),
            bottom_material: Some(bottom),
        }
    }

    // Ray-cube intersection using slab method
    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let half_size = self.size / 2.0;
        let min = self.position - Vec3::new(half_size, half_size, half_size);
        let max = self.position + Vec3::new(half_size, half_size, half_size);

        let inv_dir = Vec3::new(
            1.0 / ray.direction.x,
            1.0 / ray.direction.y,
            1.0 / ray.direction.z,
        );

        let t1 = (min.x - ray.origin.x) * inv_dir.x;
        let t2 = (max.x - ray.origin.x) * inv_dir.x;
        let t3 = (min.y - ray.origin.y) * inv_dir.y;
        let t4 = (max.y - ray.origin.y) * inv_dir.y;
        let t5 = (min.z - ray.origin.z) * inv_dir.z;
        let t6 = (max.z - ray.origin.z) * inv_dir.z;

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        if tmax < 0.0 || tmin > tmax {
            return None;
        }

        let t = if tmin > 0.001 { tmin } else { tmax };
        if t < 0.001 {
            return None;
        }

        let hit_point = ray.at(t);
        let normal = self.get_normal(hit_point, &min, &max);
        let (u, v) = self.get_uv(hit_point, &normal);

        // Select the appropriate material based on which face was hit
        let material = self.get_face_material(&normal);

        Some(Intersection::new(
            t,
            hit_point,
            normal,
            material,
            u,
            v,
        ))
    }

    // Get the material for a specific face based on the normal
    fn get_face_material(&self, normal: &Vec3) -> Material {
        // Top face (normal pointing up)
        if normal.y > 0.5 {
            if let Some(ref mat) = self.top_material {
                return mat.clone();
            }
        }
        // Bottom face (normal pointing down)
        else if normal.y < -0.5 {
            if let Some(ref mat) = self.bottom_material {
                return mat.clone();
            }
        }
        // Side faces
        else {
            if let Some(ref mat) = self.side_material {
                return mat.clone();
            }
        }

        // Fallback to default material
        self.material.clone()
    }

    fn get_normal(&self, point: Vec3, min: &Vec3, max: &Vec3) -> Vec3 {
        let epsilon = 0.001;

        if (point.x - min.x).abs() < epsilon { Vec3::new(-1.0, 0.0, 0.0) }
        else if (point.x - max.x).abs() < epsilon { Vec3::new(1.0, 0.0, 0.0) }
        else if (point.y - min.y).abs() < epsilon { Vec3::new(0.0, -1.0, 0.0) }
        else if (point.y - max.y).abs() < epsilon { Vec3::new(0.0, 1.0, 0.0) }
        else if (point.z - min.z).abs() < epsilon { Vec3::new(0.0, 0.0, -1.0) }
        else { Vec3::new(0.0, 0.0, 1.0) }
    }

    fn get_uv(&self, point: Vec3, normal: &Vec3) -> (f32, f32) {
        let local = point - self.position;
        let half_size = self.size / 2.0;

        let u: f32;
        let v: f32;

        if normal.x.abs() > 0.5 {
            u = (local.z + half_size) / self.size;
            v = (local.y + half_size) / self.size;
        } else if normal.y.abs() > 0.5 {
            u = (local.x + half_size) / self.size;
            v = (local.z + half_size) / self.size;
        } else {
            u = (local.x + half_size) / self.size;
            v = (local.y + half_size) / self.size;
        }

        (u, v)
    }
}
