use raylib::prelude::*;

mod camera;
mod ray;
mod material;
mod texture;
mod color;
mod scene;
mod cube;
mod light;
mod skybox;
mod obj_loader;
mod intersection;
mod renderer;
mod utils;

use camera::Camera;
use scene::Scene;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Minecraft Raytracer - Diorama")
        .build();

    rl.set_target_fps(60);

    let mut scene = Scene::new();
    scene.build_cherry_tree_diorama();

    let mut camera = Camera::new(
        utils::Vec3::new(0.0, 5.0, 15.0),
        utils::Vec3::new(0.0, 0.0, 0.0),
        70.0,
        WIDTH as f32 / HEIGHT as f32,
    );

    let mut quality_level = 1;
    let mut manual_quality_level = 1; // User's preferred quality
    let mut use_threading = true;
    let mut day_time = 0.0f32;
    let mut auto_quality = false; // Auto performance scaling

    // FPS tracking for auto quality
    let mut fps_history: Vec<u32> = Vec::new();
    let mut fps_check_timer = 0.0f32;
    const FPS_CHECK_INTERVAL: f32 = 0.5; // Check FPS every 0.5 seconds
    const LOW_FPS_THRESHOLD: u32 = 20;
    const HIGH_FPS_THRESHOLD: u32 = 45;

    let mut image_buffer = vec![Color::BLACK; (WIDTH * HEIGHT) as usize];

    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();
        let current_fps = rl.get_fps();

        handle_camera_input(&rl, &mut camera, delta_time);

        // === Quality Control ===
        if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
            manual_quality_level = 0;
            if !auto_quality { quality_level = 0; }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
            manual_quality_level = 1;
            if !auto_quality { quality_level = 1; }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
            manual_quality_level = 2;
            if !auto_quality { quality_level = 2; }
        }

        // Toggle auto performance mode
        if rl.is_key_pressed(KeyboardKey::KEY_P) {
            auto_quality = !auto_quality;
            if !auto_quality {
                quality_level = manual_quality_level; // Restore manual quality
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_T) { use_threading = !use_threading; }

        if rl.is_key_down(KeyboardKey::KEY_N) {
            day_time = (day_time + 0.01) % 1.0;
        }

        // === Auto Quality Adjustment ===
        if auto_quality {
            fps_check_timer += delta_time;

            // Track FPS history
            fps_history.push(current_fps);
            if fps_history.len() > 10 {
                fps_history.remove(0);
            }

            // Adjust quality based on average FPS
            if fps_check_timer >= FPS_CHECK_INTERVAL && fps_history.len() >= 5 {
                fps_check_timer = 0.0;

                let avg_fps: u32 = fps_history.iter().sum::<u32>() / fps_history.len() as u32;

                // Lower quality if FPS is too low
                if avg_fps < LOW_FPS_THRESHOLD && quality_level < 2 {
                    quality_level += 1;
                    println!("Auto-scaling: Lowering quality to improve FPS (avg: {})", avg_fps);
                }
                // Raise quality if FPS is consistently high
                else if avg_fps > HIGH_FPS_THRESHOLD && quality_level > 0 {
                    // Only increase if we can maintain good FPS
                    if quality_level > manual_quality_level {
                        quality_level -= 1;
                        println!("Auto-scaling: Raising quality (avg: {})", avg_fps);
                    }
                }
            }
        }

        scene.update_sun_position(day_time);

        let render_scale = match quality_level {
            0 => 4,  // Low: 4x downscale (1/16th pixels)
            1 => 2,  // Medium: 2x downscale (1/4th pixels)
            _ => 1,  // High: Native resolution
        };

        renderer::render_scene(
            &scene,
            &camera,
            &mut image_buffer,
            WIDTH,
            HEIGHT,
            render_scale,
            use_threading,
        );

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        draw_buffer(&mut d, &image_buffer, WIDTH, HEIGHT);

        // === Performance Display ===
        let fps = d.get_fps();
        let fps_color = if fps >= 50 {
            Color::GREEN
        } else if fps >= 25 {
            Color::YELLOW
        } else {
            Color::RED
        };
        d.draw_text(&format!("FPS: {}", fps), 10, 10, 20, fps_color);

        // Quality display with color coding
        let (quality_text, quality_color) = match quality_level {
            0 => ("Low (4x)", Color::ORANGE),
            1 => ("Medium (2x)", Color::SKYBLUE),
            _ => ("High (1x)", Color::LIME),
        };
        d.draw_text(&format!("Quality: {}", quality_text), 10, 35, 20, quality_color);

        // Show auto-quality status
        if auto_quality {
            d.draw_text("[AUTO PERF]", 200, 35, 20, Color::GOLD);
        }

        // Render scale info
        let pixels_rendered = ((WIDTH * HEIGHT) / (render_scale * render_scale)) as f32;
        let percentage = (pixels_rendered / (WIDTH * HEIGHT) as f32) * 100.0;
        d.draw_text(
            &format!("Pixels: {:.0}% ({}/{})", percentage, pixels_rendered as i32, WIDTH * HEIGHT),
            10, 60,
            16,
            Color::LIGHTGRAY,
        );

        d.draw_text(&format!("Threading: {}", if use_threading { "ON" } else { "OFF" }), 10, 85, 16, Color::WHITE);
        d.draw_text(&format!("Day Time: {:.2}", day_time), 10, 105, 16, Color::YELLOW);

        // Controls display
        d.draw_text("Controls:", 10, HEIGHT - 100, 16, Color::LIGHTGRAY);
        d.draw_text("WASD: Move | Arrows L/R: Rotate | Arrows U/D: Zoom", 10, HEIGHT - 80, 16, Color::LIGHTGRAY);
        d.draw_text("Q/E: Up/Down | 1/2/3: Quality | P: Auto-Perf | T: Threading", 10, HEIGHT - 60, 16, Color::LIGHTGRAY);
        d.draw_text("N: Day/Night", 10, HEIGHT - 40, 16, Color::LIGHTGRAY);
    }
}

fn handle_camera_input(rl: &RaylibHandle, camera: &mut Camera, delta_time: f32) {
    // Movement speed (units per second)
    let move_speed = 5.0;
    let rotation_speed = 60.0; // degrees per second
    let zoom_speed = 10.0;
    let vertical_speed = 5.0;

    // Calculate movement amounts based on delta_time for smooth, frame-rate independent movement
    let move_amount = move_speed * delta_time;
    let rotate_amount = rotation_speed * delta_time;
    let zoom_amount = zoom_speed * delta_time;
    let vertical_amount = vertical_speed * delta_time;

    // === WASD Movement ===
    if rl.is_key_down(KeyboardKey::KEY_W) {
        camera.move_forward(move_amount);
    }
    if rl.is_key_down(KeyboardKey::KEY_S) {
        camera.move_backward(move_amount);
    }
    if rl.is_key_down(KeyboardKey::KEY_A) {
        camera.strafe_left(move_amount);
    }
    if rl.is_key_down(KeyboardKey::KEY_D) {
        camera.strafe_right(move_amount);
    }

    // === Arrow Keys - Rotation ===
    if rl.is_key_down(KeyboardKey::KEY_LEFT) {
        camera.rotate_around_target(-rotate_amount);
    }
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
        camera.rotate_around_target(rotate_amount);
    }

    // === Arrow Keys - Zoom ===
    if rl.is_key_down(KeyboardKey::KEY_UP) {
        camera.zoom(-zoom_amount);
    }
    if rl.is_key_down(KeyboardKey::KEY_DOWN) {
        camera.zoom(zoom_amount);
    }

    // === Q/E - Vertical Movement ===
    if rl.is_key_down(KeyboardKey::KEY_Q) {
        camera.move_up(vertical_amount);
    }
    if rl.is_key_down(KeyboardKey::KEY_E) {
        camera.move_down(vertical_amount);
    }
}

fn draw_buffer(d: &mut RaylibDrawHandle, buffer: &[Color], width: i32, height: i32) {
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            d.draw_pixel(x, y, buffer[idx]);
        }
    }
}
