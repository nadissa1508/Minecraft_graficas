use crate::utils::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use crate::intersection::Intersection;

pub struct Triangle {
    pub v0: Vec3,
    pub v1: Vec3,
    pub v2: Vec3,
    pub normal: Vec3,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3) -> Self {
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let normal = edge1.cross(&edge2).normalize();

        Self { v0, v1, v2, normal }
    }

    // Möller-Trumbore intersection algorithm
    pub fn intersect(&self, ray: &Ray) -> Option<f32> {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let h = ray.direction.cross(&edge2);
        let a = edge1.dot(&h);

        if a.abs() < 0.00001 {
            return None;
        }

        let f = 1.0 / a;
        let s = ray.origin - self.v0;
        let u = f * s.dot(&h);

        if u < 0.0 || u > 1.0 {
            return None;
        }

        let q = s.cross(&edge1);
        let v = f * ray.direction.dot(&q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(&q);

        if t > 0.001 {
            Some(t)
        } else {
            None
        }
    }
}

pub struct Mesh {
    pub triangles: Vec<Triangle>,
    pub position: Vec3,
    pub material: Material,
}

impl Mesh {
    pub fn new(position: Vec3, material: Material) -> Self {
        Self {
            triangles: Vec::new(),
            position,
            material,
        }
    }

    // TODO: Implement actual .OBJ file loading
    pub fn load_obj(_path: &str, position: Vec3, material: Material) -> Self {
        // Placeholder: Create a simple pyramid
        let triangles = vec![
            Triangle::new(
                Vec3::new(-0.5, 0.0, -0.5),
                Vec3::new(0.5, 0.0, -0.5),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            Triangle::new(
                Vec3::new(0.5, 0.0, -0.5),
                Vec3::new(0.5, 0.0, 0.5),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            Triangle::new(
                Vec3::new(0.5, 0.0, 0.5),
                Vec3::new(-0.5, 0.0, 0.5),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            Triangle::new(
                Vec3::new(-0.5, 0.0, 0.5),
                Vec3::new(-0.5, 0.0, -0.5),
                Vec3::new(0.0, 1.0, 0.0),
            ),
        ];

        Self {
            triangles,
            position,
            material,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut closest_t = f32::INFINITY;
        let mut closest_triangle: Option<&Triangle> = None;

        // Transform ray to local space
        let local_ray = Ray::new(ray.origin - self.position, ray.direction);

        for triangle in &self.triangles {
            if let Some(t) = triangle.intersect(&local_ray) {
                if t < closest_t {
                    closest_t = t;
                    closest_triangle = Some(triangle);
                }
            }
        }

        closest_triangle.map(|tri| {
            let hit_point = ray.at(closest_t);
            Intersection::new(
                closest_t,
                hit_point,
                tri.normal,
                self.material.clone(),
                0.0,
                0.0,
            )
        })
    }
}
