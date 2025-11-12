use crate::cube::Cube;
use crate::obj_loader::Mesh;
use crate::light::DirectionalLight;
use crate::skybox::Skybox;
use crate::material::Material;
use crate::texture::Texture;
use crate::color::Color;
use crate::utils::Vec3;
use crate::ray::Ray;
use crate::intersection::Intersection;

pub struct Scene {
    pub cubes: Vec<Cube>,
    pub meshes: Vec<Mesh>,
    pub sun: DirectionalLight,
    pub skybox: Skybox,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            cubes: Vec::new(),
            meshes: Vec::new(),
            // Sun direction points downward at 45° angle (will be negated in renderer)
            // When negated: points up and to the right at 45°, lighting both tops and sides
            sun: DirectionalLight::sun(Vec3::new(-1.0, -1.0, -0.5).normalize(), 1.2),
            skybox: Skybox::new(),
        }
    }

    pub fn build_cherry_tree_diorama(&mut self) {
        // Create ground plane with grass blocks (different textures per face)
        for x in -5..5 {
            for z in -5..5 {
                // Top face: grass texture
                let grass_top = Material::new(Color::new(0.3, 0.7, 0.3))
                    .with_texture(Texture::load("assets/textures/grass.jpg"));

                // Side faces: grass side texture
                let grass_side = Material::new(Color::new(0.5, 0.6, 0.4))
                    .with_texture(Texture::load("assets/textures/grass_side.jpg"));

                // Bottom face: dirt texture (use grass_side as fallback if dirt doesn't exist)
                let grass_bottom = Material::new(Color::new(0.4, 0.3, 0.2))
                    .with_texture(Texture::load("assets/textures/grass_side.jpg"));

                self.cubes.push(Cube::new_multi_texture(
                    Vec3::new(x as f32, -0.5, z as f32),
                    1.0,
                    grass_top,
                    grass_side,
                    grass_bottom,
                ));
            }
        }

        // Create cherry tree trunk
        let wood_mat = Material::new(Color::new(0.5, 0.3, 0.2))
            .with_texture(Texture::load("assets/textures/cherry_wood.jpg"));

        for y in 0..4 {
            self.cubes.push(Cube::new(
                Vec3::new(0.0, y as f32, 0.0),
                1.0,
                wood_mat.clone(),
            ));
        }

        // Create cherry tree leaves
        let leaves_mat = Material::new(Color::new(1.0, 0.7, 0.8))
            .with_texture(Texture::load("assets/textures/cherry_leaves.png"));

        for x in -2i32..=2 {
            for y in 3i32..=5 {
                for z in -2i32..=2 {
                    if (x.abs() + z.abs()) < 4 && y < 6 {
                        self.cubes.push(Cube::new(
                            Vec3::new(x as f32, y as f32, z as f32),
                            1.0,
                            leaves_mat.clone(),
                        ));
                    }
                }
            }
        }

        // Add a lantern
        let lantern_mat = Material::new(Color::new(1.0, 0.9, 0.6))
            .with_emissive(Color::new(1.0, 0.8, 0.4) * 2.0);

        self.cubes.push(Cube::new(
            Vec3::new(3.0, 1.0, 3.0),
            0.5,
            lantern_mat,
        ));

        // Add some stone blocks
        let stone_mat = Material::new(Color::new(0.5, 0.5, 0.5))
            .with_texture(Texture::load("assets/textures/stone.jpg"));

        self.cubes.push(Cube::new(
            Vec3::new(-3.0, 0.0, -3.0),
            1.0,
            stone_mat.clone(),
        ));

        // Add water block
        let water_mat = Material::new(Color::new(0.2, 0.4, 0.8))
            .with_texture(Texture::load("assets/textures/water.jpeg"))
            .with_transparency(0.7, 1.33);

        self.cubes.push(Cube::new(
            Vec3::new(-2.0, 0.0, 2.0),
            1.0,
            water_mat,
        ));

        // Add glass block
        let glass_mat = Material::new(Color::new(0.9, 0.9, 1.0))
            .with_texture(Texture::load("assets/textures/glass.png"))
            .with_transparency(0.9, 1.5)
            .with_reflectivity(0.1);

        self.cubes.push(Cube::new(
            Vec3::new(2.0, 0.0, -2.0),
            1.0,
            glass_mat,
        ));

        // TODO: Load axolotl model
        // let axolotl_mat = Material::new(Color::new(1.0, 0.7, 0.8));
        // let axolotl = Mesh::load_obj("assets/models/axolotl.obj", Vec3::new(0.0, 0.5, 3.0), axolotl_mat);
        // self.meshes.push(axolotl);
    }

    pub fn update_sun_position(&mut self, day_time: f32) {
        // Animate sun from east to west, arcing overhead
        let angle = day_time * std::f32::consts::PI * 2.0;

        // Sun direction at 45° angle - points DOWN and at an angle
        // When negated in renderer, points UP and at an angle
        // This lights both horizontal and vertical surfaces
        let sun_dir = Vec3::new(
            -angle.sin() * 1.0,           // East to West movement
            -(angle.cos() + 0.5).max(0.3), // Downward at 45° (becomes upward when negated)
            -0.5,                          // Slightly north
        ).normalize();

        // Intensity based on sun height
        let sun_height = (angle.cos() + 0.5).max(0.0);
        let intensity = (sun_height * 1.2).min(1.2).max(0.3);

        self.sun = DirectionalLight::sun(sun_dir, intensity);
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut closest: Option<Intersection> = None;
        let mut closest_t = f32::INFINITY;

        // Check cubes
        for cube in &self.cubes {
            if let Some(intersection) = cube.intersect(ray) {
                if intersection.t < closest_t {
                    closest_t = intersection.t;
                    closest = Some(intersection);
                }
            }
        }

        // Check meshes
        for mesh in &self.meshes {
            if let Some(intersection) = mesh.intersect(ray) {
                if intersection.t < closest_t {
                    closest_t = intersection.t;
                    closest = Some(intersection);
                }
            }
        }

        closest
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
