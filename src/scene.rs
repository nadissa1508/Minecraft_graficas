use crate::color::Color;
use crate::cube::Cube;
use crate::intersection::Intersection;
use crate::light::DirectionalLight;
use crate::material::Material;
use crate::obj_loader::Mesh;
use crate::point_light::PointLight;
use crate::ray::Ray;
use crate::skybox::Skybox;
use crate::texture::Texture;
use crate::utils::Vec3;

pub struct Scene {
    pub cubes: Vec<Cube>,
    pub meshes: Vec<Mesh>,
    pub sun: DirectionalLight,
    pub point_lights: Vec<PointLight>,
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
            point_lights: Vec::new(),
            skybox: Skybox::new(),
        }
    }

    pub fn build_cherry_tree_diorama(&mut self) {
        // === ADD DIRT LAYER UNDER GRASS ===
        // Create dirt blocks underneath the entire diorama
        let dirt_mat = Material::new(Color::new(0.4, 0.3, 0.2))
            .with_texture(Texture::load("assets/textures/dirt.jpg"));

        for x in -10..10 {
            for z in -15..6 {
                self.cubes.push(Cube::new(
                    Vec3::new(x as f32, -1.5, z as f32),
                    1.0,
                    dirt_mat.clone(),
                ));
            }
        }

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

        // Build cherry trees
        self.build_cherry_tree(0.0, -1.0);  // Original tree at the center
        self.build_cherry_tree(7.0, -4.0);  // Second tree behind the pond

        // === BUILD CEMENT SIDEWALK NEAR HOUSE ===
        let stone_mat = Material::new(Color::new(0.6, 0.6, 0.6))
            .with_texture(Texture::load("assets/textures/stone.jpg"))
            .with_reflectivity(0.02)
            .with_specular(0.2, 16.0);  // Dull, soft highlights on stone

        // House is at x=-10 to -4, z=-10 to -4
        // Create sidewalk around the house (2 blocks wide)
        
        // Front sidewalk (along z = -10 side, extending to grass edge)
        for x in -12..=10 {
            for z in -14..=-11 {
                self.cubes.push(Cube::new(
                    Vec3::new(x as f32, 0.0, z as f32),
                    1.0,
                    stone_mat.clone(),
                ));
            }
        }
        
        // Right side sidewalk (along x = -3 side)
        for x in -4..=-2 {
            for z in -10..=-2 {
                self.cubes.push(Cube::new(
                    Vec3::new(x as f32, 0.0, z as f32),
                    1.0,
                    stone_mat.clone(),
                ));
            }
        }
        
        // Back sidewalk (along z = -3 side)
        for x in -10..=-2 {
            for z in -3..=-2 {
                self.cubes.push(Cube::new(
                    Vec3::new(x as f32, 0.0, z as f32),
                    1.0,
                    stone_mat.clone(),
                ));
            }
        }

        // === ADD GRASS UNDER HOUSE ===
        // Fill in grass blocks under the house area so it doesn't look floating
        let grass_mat = Material::new(Color::new(0.3, 0.7, 0.3))
            .with_texture(Texture::load("assets/textures/grass.jpg"));
        
        // House occupies x: -10 to -4, z: -10 to -4
        for x in -10..=-4 {
            for z in -10..=-4 {
                self.cubes.push(Cube::new(
                    Vec3::new(x as f32, -0.5, z as f32),
                    1.0,
                    grass_mat.clone(),
                ));
            }
        }

        // Add glass block
        let glass_mat = Material::new(Color::new(0.9, 0.9, 1.0))
            .with_texture(Texture::load("assets/textures/glass.png"))
            .with_transparency(0.9, 1.5)
            .with_reflectivity(0.1)
            .with_specular(0.9, 128.0);  // Very sharp, bright highlights on glass

        self.cubes
            .push(Cube::new(Vec3::new(2.0, 0.0, -2.0), 1.0, glass_mat));

        // === ADD METALLIC/GOLD DECORATIVE BLOCKS ===
        // Gold material: Very high specular for shiny metal appearance
        let gold_mat = Material::new(Color::new(1.0, 0.84, 0.0))
            .with_texture(Texture::load("assets/textures/wood.png"))  // Using wood texture as fallback
            .with_reflectivity(0.4)
            .with_specular(1.0, 256.0);  // Very sharp, intense highlights for metallic look

        // Place decorative gold blocks (removed the one at 4,0,0 that was near pond)
        self.cubes.push(Cube::new(Vec3::new(4.0, 1.0, 0.0), 1.0, gold_mat.clone()));
        self.cubes.push(Cube::new(Vec3::new(-4.0, 0.0, -4.0), 1.0, gold_mat));

        // Load axolotl model with pink material (smaller size, rotated 180°)
        let axolotl_body_mat = Material::new(Color::new(1.0, 0.7, 0.8)); // Pink/rosado body color
        let mut axolotl = Mesh::load_obj(
            "assets/models/axolotl.obj",
            Vec3::new(-1.0, 0.2, 4.0), // Position: near the tree
            0.15,                     // Scale: 0.15 = 15% of original size (much smaller!)
            axolotl_body_mat,
        );
        // Rotate 180 degrees around Y axis
        axolotl.rotate_y(std::f32::consts::PI);
        self.meshes.push(axolotl);

        // === ADD AXOLOTL FEATURES ===
        // Eyes (big, bright, and emissive so they're clearly visible!)
        let eye_mat = Material::new(Color::new(0.05, 0.05, 0.05)) // Very dark
            .with_emissive(Color::new(0.1, 0.1, 0.1)); // Slight glow to stand out
        
        // Make eyes MUCH bigger and position them at the front
        self.cubes.push(Cube::new(Vec3::new(-1.15, 0.5, 3.75), 0.18, eye_mat.clone())); // Left eye - bigger!
        self.cubes.push(Cube::new(Vec3::new(-0.85, 0.5, 3.75), 0.18, eye_mat));         // Right eye - bigger!

        // Mouth (darker pink, more visible) 
        let mouth_mat = Material::new(Color::new(0.7, 0.3, 0.4)) // Darker, more contrast
            .with_emissive(Color::new(0.1, 0.05, 0.05)); // Slight glow
        self.cubes.push(Cube::new(Vec3::new(-1.0, 0.35, 3.65), 0.15, mouth_mat));

        // Scales/Gills (bright pink frills on sides) - adjusted positions
        let scale_mat = Material::new(Color::new(1.0, 0.4, 0.6)) // Brighter pink for gills
            .with_emissive(Color::new(0.3, 0.1, 0.15)); // More visible glow
        
        // Left gills (3 small cubes) - adjusted for rotation
        self.cubes.push(Cube::new(Vec3::new(-1.3, 0.4, 4.0), 0.08, scale_mat.clone()));
        self.cubes.push(Cube::new(Vec3::new(-1.35, 0.45, 4.0), 0.07, scale_mat.clone()));
        self.cubes.push(Cube::new(Vec3::new(-1.35, 0.35, 4.0), 0.07, scale_mat.clone()));
        
        // Right gills (3 small cubes) - adjusted for rotation
        self.cubes.push(Cube::new(Vec3::new(-0.7, 0.4, 4.0), 0.08, scale_mat.clone()));
        self.cubes.push(Cube::new(Vec3::new(-0.65, 0.45, 4.0), 0.07, scale_mat.clone()));
        self.cubes.push(Cube::new(Vec3::new(-0.65, 0.35, 4.0), 0.07, scale_mat));

        // === BUILD POND AND FOUNTAIN ===
        self.build_pond();

        // === BUILD A HOUSE ===
        self.build_house();
    }

    fn build_cherry_tree(&mut self, base_x: f32, base_z: f32) {
        // Create cherry tree trunk
        let wood_mat = Material::new(Color::new(0.5, 0.3, 0.2))
            .with_texture(Texture::load("assets/textures/cherry_wood.jpg"))
            .with_specular(0.1, 32.0);  // Minimal, soft highlights on wood

        for y in 0..4 {
            self.cubes.push(Cube::new(
                Vec3::new(base_x, y as f32, base_z),
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
                            Vec3::new(base_x + x as f32, y as f32, base_z + z as f32),
                            1.0,
                            leaves_mat.clone(),
                        ));
                    }
                }
            }
        }
    }

    fn build_pond(&mut self) {
        // Pond position (rectangular pond near the tree and axolotl)
        let pond_center_x = 5.0;
        let pond_center_z = 2.0;
        
        // Pond dimensions (create a regular 5x4 rectangular pond)
        let pond_width = 5;  // Width along x-axis
        let pond_depth = 4;  // Depth along z-axis

        // === POND MATERIALS ===
        let water_mat = Material::new(Color::new(0.2, 0.5, 0.9))
            .with_texture(Texture::load("assets/textures/water.jpeg"))
            .with_transparency(0.85, 1.33)
            .with_reflectivity(0.3)
            .with_specular(0.8, 64.0);  // Strong, sharp highlights on water

        let stone_mat = Material::new(Color::new(0.5, 0.5, 0.5))
            .with_texture(Texture::load("assets/textures/stone.jpg"))
            .with_reflectivity(0.05);

        // Lily pad material (green, for decoration)
        let lily_mat = Material::new(Color::new(0.3, 0.7, 0.3))
            .with_texture(Texture::load("assets/textures/grass.jpg"));

        // === CREATE RECTANGULAR POND ===
        // Calculate starting corner
        let start_x = pond_center_x - (pond_width as f32 / 2.0);
        let start_z = pond_center_z - (pond_depth as f32 / 2.0);

        // Create stone border (outer ring)
        for x in -1..=pond_width {
            for z in -1..=pond_depth {
                let x_pos = start_x + x as f32;
                let z_pos = start_z + z as f32;
                
                // Only place stones on the border
                if x == -1 || x == pond_width || z == -1 || z == pond_depth {
                    self.cubes.push(Cube::new(
                        Vec3::new(x_pos, 0.0, z_pos),
                        1.0,
                        stone_mat.clone(),
                    ));
                }
            }
        }

        // Fill interior with water
        for x in 0..pond_width {
            for z in 0..pond_depth {
                let x_pos = start_x + x as f32;
                let z_pos = start_z + z as f32;
                
                self.cubes.push(Cube::new(
                    Vec3::new(x_pos, 0.0, z_pos),
                    1.0,
                    water_mat.clone(),
                ));
            }
        }

        // === ADD LILY PADS (optional decoration) ===
        // Place a few lily pads floating on the water surface
        self.cubes.push(Cube::new(
            Vec3::new(pond_center_x - 1.0, 0.9, pond_center_z - 0.5),
            0.4,
            lily_mat.clone(),
        ));

        self.cubes.push(Cube::new(
            Vec3::new(pond_center_x + 1.0, 0.9, pond_center_z + 0.5),
            0.4,
            lily_mat.clone(),
        ));
        
        self.cubes.push(Cube::new(
            Vec3::new(pond_center_x, 0.9, pond_center_z),
            0.4,
            lily_mat,
        ));
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

        let door_mat = Material::new(Color::new(0.5, 0.5, 0.5))
            .with_texture(Texture::load("assets/textures/wood.png"));

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

            // Front wall (z = house_z) with windows
            for x in 0..house_width {
                let x_pos = house_x + x as f32;
                let is_window = y >= 2 && y <= 3 && (x == 2 || x == 4);

                if is_window {
                    self.cubes.push(Cube::new(
                        Vec3::new(x_pos, y_pos, house_z),
                        1.0,
                        window_mat.clone(),
                    ));
                } else {
                    self.cubes.push(Cube::new(
                        Vec3::new(x_pos, y_pos, house_z),
                        1.0,
                        wall_mat.clone(),
                    ));
                }
            }

            // Back wall (z = house_z + depth) with door
            for x in 0..house_width {
                let x_pos = house_x + x as f32;
                let is_door = y < 3 && x >= 2 && x <= 4; // Door opening (3 blocks wide, 3 blocks tall)

                if !is_door {
                    self.cubes.push(Cube::new(
                        Vec3::new(x_pos, y_pos, house_z + house_depth as f32 - 1.0),
                        1.0,
                        wall_mat.clone(),
                    ));
                } else {
                    // Door blocks filling entire 3x3 opening
                    self.cubes.push(Cube::new(
                        Vec3::new(x_pos, y_pos, house_z + house_depth as f32 - 1.0),
                        1.0,
                        door_mat.clone(),
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
            -angle.sin() * 1.0,            // East to West movement
            -(angle.cos() + 0.5).max(0.3), // Downward at 45° (becomes upward when negated)
            -0.5,                          // Slightly north
        )
        .normalize();

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
