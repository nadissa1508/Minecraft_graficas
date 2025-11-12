use crate::color::Color;
use crate::texture::Texture;

#[derive(Clone)]
pub struct Material {
    pub albedo: Color,
    pub texture: Option<Texture>,
    pub reflectivity: f32,
    pub emissive: Color,
    pub refractive_index: f32,
    pub transparency: f32,
}

impl Material {
    pub fn new(albedo: Color) -> Self {
        Self {
            albedo,
            texture: None,
            reflectivity: 0.0,
            emissive: Color::black(),
            refractive_index: 1.0,
            transparency: 0.0,
        }
    }

    pub fn with_texture(mut self, texture: Texture) -> Self {
        self.texture = Some(texture);
        self
    }

    pub fn with_reflectivity(mut self, reflectivity: f32) -> Self {
        self.reflectivity = reflectivity;
        self
    }

    pub fn with_emissive(mut self, emissive: Color) -> Self {
        self.emissive = emissive;
        self
    }

    pub fn with_transparency(mut self, transparency: f32, refractive_index: f32) -> Self {
        self.transparency = transparency;
        self.refractive_index = refractive_index;
        self
    }

    pub fn get_color(&self, u: f32, v: f32) -> Color {
        if let Some(ref texture) = self.texture {
            texture.sample(u, v)
        } else {
            self.albedo
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new(Color::white())
    }
}
