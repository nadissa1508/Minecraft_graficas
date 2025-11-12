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
        // Asymmetric: more grass in front of house (negative z), less behind axolotl (positive z)
        for x in -10..10 {
            for z in -15..6 {
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

        // Load axolotl model with pink material (smaller size)
        let axolotl_body_mat = Material::new(Color::new(1.0, 0.7, 0.8)); // Pink/rosado body color
        let axolotl = Mesh::load_obj(
            "assets/models/axolotl.obj",
            Vec3::new(2.0, 0.2, 4.0), // Position: near the tree
            0.15,                      // Scale: 0.15 = 15% of original size (much smaller!)
            axolotl_body_mat,
        );
        self.meshes.push(axolotl);

        // TODO: Add eyes, mouth, and scales as separate colored cubes or small meshes
        // Eyes (dark spots)
        // let eye_mat = Material::new(Color::new(0.1, 0.1, 0.1)); // Dark for eyes
        // self.cubes.push(Cube::new(Vec3::new(-0.1, 0.6, 3.3), 0.08, eye_mat.clone()));
        // self.cubes.push(Cube::new(Vec3::new(0.1, 0.6, 3.3), 0.08, eye_mat));

        // Mouth (darker pink)
        // let mouth_mat = Material::new(Color::new(0.8, 0.4, 0.5));
        // self.cubes.push(Cube::new(Vec3::new(0.0, 0.4, 3.4), 0.05, mouth_mat));

        // === BUILD A HOUSE ===
        self.build_house();
    }

    fn build_house(&mut self) {
        // House materials
        let wall_mat = Material::new(Color::new(0.6, 0.4, 0.3))
            .with_texture(Texture::load("assets/textures/cherry_log.png"));

        let window_mat = Material::new(Color::new(0.8, 0.9, 1.0))
            .with_texture(Texture::load("assets/textures/glass.png"))
            .with_transparency(0.8, 1.5)
            .with_reflectivity(0.1);

        let roof_mat = Material::new(Color::new(0.5, 0.5, 0.5))
            .with_texture(Texture::load("assets/textures/stone.jpg"));

        let door_mat = Material::new(Color::new(0.5, 0.3, 0.2))
            .with_texture(Texture::load("assets/textures/cherry_wood.jpg"));

        // House position and size
        let house_x = -10.0;
        let house_z = -10.0;
        let house_width = 7;
        let house_depth = 7;
        let house_height = 5;

        // Build floor (optional, grass is already there)

        // Build walls (all 4 sides)
        for y in 0..house_height {
            let y_pos = y as f32;

            // Front wall (z = house_z) with door
            for x in 0..house_width {
                let x_pos = house_x + x as f32;
                let is_door = y < 3 && x >= 2 && x <= 4; // Door opening (3 blocks wide, 3 blocks tall)

                if !is_door {
                    self.cubes.push(Cube::new(
                        Vec3::new(x_pos, y_pos, house_z),
                        1.0,
                        wall_mat.clone(),
                    ));
                } else if y == 0 {
                    // Door block at ground level
                    self.cubes.push(Cube::new(
                        Vec3::new(x_pos, y_pos, house_z),
                        1.0,
                        door_mat.clone(),
                    ));
                }
            }

            // Back wall (z = house_z + depth) with windows
            for x in 0..house_width {
                let x_pos = house_x + x as f32;
                let is_window = y >= 2 && y <= 3 && (x == 2 || x == 4);

                if is_window {
                    self.cubes.push(Cube::new(
                        Vec3::new(x_pos, y_pos, house_z + house_depth as f32 - 1.0),
                        1.0,
                        window_mat.clone(),
                    ));
                } else {
                    self.cubes.push(Cube::new(
                        Vec3::new(x_pos, y_pos, house_z + house_depth as f32 - 1.0),
                        1.0,
                        wall_mat.clone(),
                    ));
                }
            }

            // Left wall (x = house_x) with window
            for z in 1..(house_depth - 1) {
                let z_pos = house_z + z as f32;
                let is_window = y >= 2 && y <= 3 && z == 3;

                if is_window {
                    self.cubes.push(Cube::new(
                        Vec3::new(house_x, y_pos, z_pos),
                        1.0,
                        window_mat.clone(),
                    ));
                } else {
                    self.cubes.push(Cube::new(
                        Vec3::new(house_x, y_pos, z_pos),
                        1.0,
                        wall_mat.clone(),
                    ));
                }
            }

            // Right wall (x = house_x + width) with window
            for z in 1..(house_depth - 1) {
                let z_pos = house_z + z as f32;
                let is_window = y >= 2 && y <= 3 && z == 3;

                if is_window {
                    self.cubes.push(Cube::new(
                        Vec3::new(house_x + house_width as f32 - 1.0, y_pos, z_pos),
                        1.0,
                        window_mat.clone(),
                    ));
                } else {
                    self.cubes.push(Cube::new(
                        Vec3::new(house_x + house_width as f32 - 1.0, y_pos, z_pos),
                        1.0,
                        wall_mat.clone(),
                    ));
                }
            }
        }

        // Build roof (flat roof made of stone)
        let roof_y = house_height as f32;
        for x in 0..house_width {
            for z in 0..house_depth {
                self.cubes.push(Cube::new(
                    Vec3::new(house_x + x as f32, roof_y, house_z + z as f32),
                    1.0,
                    roof_mat.clone(),
                ));
            }
        }
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
