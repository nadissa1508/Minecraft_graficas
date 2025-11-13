use crate::color::Color;
use crate::utils::clamp;
use image::GenericImageView;

#[derive(Clone)]
pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Color>,
}

impl Texture {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![Color::white(); width * height],
        }
    }

    pub fn from_color(color: Color) -> Self {
        Self {
            width: 1,
            height: 1,
            data: vec![color],
        }
    }

    /// Create a gradient skybox texture for day
    pub fn create_day_skybox() -> Self {
        let width = 512;
        let height = 256;
        let mut data = Vec::with_capacity(width * height);

        // Day skybox: gradient from horizon (bottom) to sky (top)
        let horizon_color = Color::new(0.8, 0.9, 1.0);    // Light blue
        let sky_color = Color::new(0.4, 0.6, 0.95);       // Sky blue

        for y in 0..height {
            let t = y as f32 / height as f32;
            for _x in 0..width {
                // Vertical gradient
                let r = horizon_color.r + (sky_color.r - horizon_color.r) * t;
                let g = horizon_color.g + (sky_color.g - horizon_color.g) * t;
                let b = horizon_color.b + (sky_color.b - horizon_color.b) * t;
                data.push(Color::new(r, g, b));
            }
        }

        println!("Created procedural day skybox texture ({}x{})", width, height);

        Self {
            width,
            height,
            data,
        }
    }

    /// Create a gradient skybox texture for night
    pub fn create_night_skybox() -> Self {
        let width = 512;
        let height = 256;
        let mut data = Vec::with_capacity(width * height);

        // Night skybox: gradient from dark horizon to darker sky with stars
        let horizon_color = Color::new(0.1, 0.1, 0.2);    // Dark blue
        let sky_color = Color::new(0.02, 0.02, 0.1);      // Very dark blue

        for y in 0..height {
            let t = y as f32 / height as f32;
            for x in 0..width {
                // Base vertical gradient
                let r = horizon_color.r + (sky_color.r - horizon_color.r) * t;
                let g = horizon_color.g + (sky_color.g - horizon_color.g) * t;
                let b = horizon_color.b + (sky_color.b - horizon_color.b) * t;

                // Add stars (simple noise-based stars)
                let star_threshold = 0.995;
                let noise = ((x * 12345 + y * 67890) % 10000) as f32 / 10000.0;

                let mut color = Color::new(r, g, b);
                if noise > star_threshold && t > 0.3 {
                    // Add a star
                    let brightness = (noise - star_threshold) / (1.0 - star_threshold);
                    color.r += brightness * 0.8;
                    color.g += brightness * 0.8;
                    color.b += brightness * 0.8;
                }

                data.push(color);
            }
        }

        println!("Created procedural night skybox texture with stars ({}x{})", width, height);

        Self {
            width,
            height,
            data,
        }
    }

    pub fn load(path: &str) -> Self {
        // Try to load the image file
        match image::open(path) {
            Ok(img) => {
                let (width, height) = img.dimensions();
                let width = width as usize;
                let height = height as usize;
                let mut data = Vec::with_capacity(width * height);

                // Convert image to RGB8 format
                let img_rgb = img.to_rgb8();

                // Load pixel data
                for y in 0..height {
                    for x in 0..width {
                        let pixel = img_rgb.get_pixel(x as u32, y as u32);
                        let color = Color::new(
                            pixel[0] as f32 / 255.0,
                            pixel[1] as f32 / 255.0,
                            pixel[2] as f32 / 255.0,
                        );
                        data.push(color);
                    }
                }

                println!("Loaded texture: {} ({}x{})", path, width, height);

                Self {
                    width,
                    height,
                    data,
                }
            }
            Err(e) => {
                eprintln!("Failed to load texture '{}': {}", path, e);
                eprintln!("Using fallback checkerboard pattern");

                // Fallback: Create a checkerboard pattern
                let width = 64;
                let height = 64;
                let mut data = Vec::with_capacity(width * height);

                for y in 0..height {
                    for x in 0..width {
                        let checker = ((x / 8) + (y / 8)) % 2 == 0;
                        let color = if checker {
                            Color::new(0.8, 0.8, 0.8)
                        } else {
                            Color::new(0.6, 0.6, 0.6)
                        };
                        data.push(color);
                    }
                }

                Self {
                    width,
                    height,
                    data,
                }
            }
        }
    }

    pub fn sample(&self, u: f32, v: f32) -> Color {
        let u = clamp(u, 0.0, 1.0);
        let v = clamp(v, 0.0, 1.0);

        let x = (u * self.width as f32) as usize;
        let y = (v * self.height as f32) as usize;

        let x = x.min(self.width - 1);
        let y = y.min(self.height - 1);

        self.data[y * self.width + x]
    }
}
