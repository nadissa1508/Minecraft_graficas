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
    pub scale: f32,
    pub material: Material,
}

impl Mesh {
    pub fn new(position: Vec3, material: Material) -> Self {
        Self {
            triangles: Vec::new(),
            position,
            scale: 1.0,
            material,
        }
    }

    /// Load an OBJ file and create a mesh with scale
    pub fn load_obj(path: &str, position: Vec3, scale: f32, material: Material) -> Self {
        println!("Loading OBJ model: {} (scale: {})", path, scale);

        // Try to load the OBJ file using tobj
        let load_options = tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        };

        match tobj::load_obj(path, &load_options) {
            Ok((models, _materials)) => {
                let mut triangles = Vec::new();

                // Process each model in the OBJ file
                for model in models {
                    let mesh = &model.mesh;
                    let positions = &mesh.positions;
                    let indices = &mesh.indices;

                    println!("  Model '{}': {} vertices, {} triangles",
                        model.name,
                        positions.len() / 3,
                        indices.len() / 3
                    );

                    // Create triangles from indices
                    for i in (0..indices.len()).step_by(3) {
                        let idx0 = indices[i] as usize;
                        let idx1 = indices[i + 1] as usize;
                        let idx2 = indices[i + 2] as usize;

                        let v0 = Vec3::new(
                            positions[idx0 * 3] * scale,
                            positions[idx0 * 3 + 1] * scale,
                            positions[idx0 * 3 + 2] * scale,
                        );

                        let v1 = Vec3::new(
                            positions[idx1 * 3] * scale,
                            positions[idx1 * 3 + 1] * scale,
                            positions[idx1 * 3 + 2] * scale,
                        );

                        let v2 = Vec3::new(
                            positions[idx2 * 3] * scale,
                            positions[idx2 * 3 + 1] * scale,
                            positions[idx2 * 3 + 2] * scale,
                        );

                        triangles.push(Triangle::new(v0, v1, v2));
                    }
                }

                println!("Successfully loaded {} triangles", triangles.len());

                Self {
                    triangles,
                    position,
                    scale,
                    material,
                }
            }
            Err(e) => {
                eprintln!("Failed to load OBJ file '{}': {}", path, e);
                eprintln!("Creating fallback pyramid mesh");

                // Fallback: Create a simple pyramid (already scaled)
                let triangles = vec![
                    Triangle::new(
                        Vec3::new(-0.5 * scale, 0.0, -0.5 * scale),
                        Vec3::new(0.5 * scale, 0.0, -0.5 * scale),
                        Vec3::new(0.0, 1.0 * scale, 0.0),
                    ),
                    Triangle::new(
                        Vec3::new(0.5 * scale, 0.0, -0.5 * scale),
                        Vec3::new(0.5 * scale, 0.0, 0.5 * scale),
                        Vec3::new(0.0, 1.0 * scale, 0.0),
                    ),
                    Triangle::new(
                        Vec3::new(0.5 * scale, 0.0, 0.5 * scale),
                        Vec3::new(-0.5 * scale, 0.0, 0.5 * scale),
                        Vec3::new(0.0, 1.0 * scale, 0.0),
                    ),
                    Triangle::new(
                        Vec3::new(-0.5 * scale, 0.0, 0.5 * scale),
                        Vec3::new(-0.5 * scale, 0.0, -0.5 * scale),
                        Vec3::new(0.0, 1.0 * scale, 0.0),
                    ),
                ];

                Self {
                    triangles,
                    position,
                    scale,
                    material,
                }
            }
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
