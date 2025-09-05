use crate::color::Color;
use nalgebra_glm::Vec3;

#[derive(Debug, Clone, Copy)]
pub enum TextureType {
    Solid(Color),
    Checkerboard(Color, Color),
    Stripes(Color, Color),
    Brick(Color, Color),
}

pub struct Texture {
    pub texture_type: TextureType,
}

impl Texture {
    pub fn new_solid(color: Color) -> Self {
        Texture {
            texture_type: TextureType::Solid(color),
        }
    }

    pub fn new_checkerboard(color1: Color, color2: Color) -> Self {
        Texture {
            texture_type: TextureType::Checkerboard(color1, color2),
        }
    }

    pub fn new_stripes(color1: Color, color2: Color) -> Self {
        Texture {
            texture_type: TextureType::Stripes(color1, color2),
        }
    }

    pub fn new_brick(brick_color: Color, mortar_color: Color) -> Self {
        Texture {
            texture_type: TextureType::Brick(brick_color, mortar_color),
        }
    }

    pub fn get_color_at_uv(&self, u: f32, v: f32) -> Color {
        match self.texture_type {
            TextureType::Solid(color) => color,
            
            TextureType::Checkerboard(color1, color2) => {
                let scale = 8.0; // Tamaño del patrón
                let u_check = (u * scale).floor() as i32;
                let v_check = (v * scale).floor() as i32;
                
                if (u_check + v_check) % 2 == 0 {
                    color1
                } else {
                    color2
                }
            },
            
            TextureType::Stripes(color1, color2) => {
                let scale = 10.0;
                if ((u * scale).floor() as i32) % 2 == 0 {
                    color1
                } else {
                    color2
                }
            },
            
            TextureType::Brick(brick_color, mortar_color) => {
                let brick_width = 8.0;
                let brick_height = 4.0;
                let mortar_thickness = 0.1;
                
                let u_scaled = u * brick_width;
                let v_scaled = v * brick_height;
                
                let brick_u = u_scaled.fract();
                let brick_v = v_scaled.fract();
                
                // Offset para patrón de ladrillo
                let row = v_scaled.floor() as i32;
                let offset = if row % 2 == 0 { 0.0 } else { 0.5 };
                let u_offset = (u_scaled + offset).fract();
                
                if brick_u < mortar_thickness || brick_u > (1.0 - mortar_thickness) ||
                   brick_v < mortar_thickness || brick_v > (1.0 - mortar_thickness) {
                    mortar_color
                } else {
                    brick_color
                }
            }
        }
    }
}