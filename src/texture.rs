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
