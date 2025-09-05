//framebuffer.rs
use raylib::prelude::*;
use crate::color::Color;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    pub background_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let color_buffer = Image::gen_image_color(width as i32, height as i32,
            raylib::color::Color::new(background_color.r, background_color.g, background_color.b, 255));

        Framebuffer {
            width,
            height,
            color_buffer,
            background_color,
        }
    }

    pub fn point(&mut self, x: u32, y: u32, color: Color) {
        if x < self.width && y < self.height {
            self.color_buffer.draw_pixel(
                x as i32,
                y as i32,
                raylib::color::Color::new(color.r, color.g, color.b, 255),
            );
        }
    }

    pub fn swap_buffers(&self, window: &mut RaylibHandle, rl_thread: &RaylibThread) {
        if let Ok(texture) = window.load_texture_from_image(rl_thread, &self.color_buffer) {
            let mut d = window.begin_drawing(rl_thread);
            d.clear_background(raylib::color::Color::BLACK);
            d.draw_texture(&texture, 0, 0, raylib::color::Color::WHITE);
        }
    }
}
