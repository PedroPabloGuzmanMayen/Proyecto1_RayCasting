mod framebuffer;
mod color;
mod bmp;
mod maze;
mod player;
mod caster;
mod render;
mod texture;
mod music;
use crate::texture::Texture;
use crate::maze::{load_maze};
use crate::framebuffer::FrameBuffer;
use crate::color::Color;
use crate::music::AudioPlayer;
use crate::caster::{cast_ray, Intersect};
use minifb::{Window, WindowOptions, Key};
use crate::player::{Player, process_event};
use std::time::{Instant, Duration};
use nalgebra_glm::{Vec2};
use render::{render3d_with_minimap, render_menu, render_enemies};
use gilrs::{Gilrs, Button, Event};

fn main() {
    let mut gilrs = Gilrs::new().unwrap();
    let window_width = 1300;
    let window_height = 900;
    let block_size = 100;
    let level1_music = AudioPlayer::new("assets/music/Jungle.mp3");
    let mut sound_effect = AudioPlayer::new("assets/music/pasos.mp3");
    let level2_music = AudioPlayer::new("assets/music/silm.mp3");

    const EXIT_POSITION: Vec2 = Vec2::new(500.0, 300.0);

    let framebuffer_width = 1300;
    let framebuffer_height = 900;
    let mut level:usize = 0;
  
    let frame_delay = Duration::from_millis(15);
    
    let mut framebuffer = framebuffer::FrameBuffer::new(framebuffer_width, framebuffer_height);
    framebuffer.set_current_color(Color::new(50,50,100));

    
    let mut z_buffer = vec![f32::INFINITY; framebuffer.width];
    let mut enemies: Vec<Vec2> = vec![];
    let mut window = Window::new(
        "Rust Graphics - Render Loop Example",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();
    let mut player = Player {
        pos: Vec2::new(250.0, 150.0),
        a: std::f32::consts::PI/3.0,
        fov: std::f32::consts::PI/3.0,
        mouse_sensitivity: 0.002,
        last_mouse_x: window.get_mouse_pos(minifb::MouseMode::Discard).unwrap_or((0.0, 0.0)).0 as f32
    };

    let mut mode = "MENU";
    let mut last_time = Instant::now();
    let mut frame_time_accumulator = Duration::new(0, 0);
    let mut frame_count = 0;
    let mut last_key_enter = false; 
    let mut last_key_1 = false;  
    let mut last_key_2 = false;  
    let mut last_key_3 = false;  

    while window.is_open() {

        let current_time = Instant::now();
        let delta_time = current_time - last_time;
        last_time = current_time;
        
        frame_time_accumulator += delta_time;
        frame_count += 1;

        if frame_time_accumulator >= Duration::from_secs(1) {
            let fps = frame_count as f64 / frame_time_accumulator.as_secs_f64();
            window.set_title(&format!("FPS: {:.2}", fps));
            frame_time_accumulator = Duration::new(0, 0);
            frame_count = 0;
        }

        if window.is_key_down(Key::Escape){
            break;
        }

        let key_enter_pressed = window.is_key_down(Key::Enter);
        if key_enter_pressed && !last_key_enter && mode == "MENU" {
            mode = "SELECT";
        }
        last_key_enter = key_enter_pressed;

        if mode == "SELECT" {
            let key_1_pressed = window.is_key_down(Key::Key1);
            if key_1_pressed && !last_key_1 {
                mode = "GAME";
                level = 1;
                enemies = vec![Vec2::new(255.0, 255.0), Vec2::new(450.0, 450.0), Vec2::new(750.0, 750.0)];
                level1_music.play();
            }
            last_key_1 = key_1_pressed;

            let key_2_pressed = window.is_key_down(Key::Key2);
            if key_2_pressed && !last_key_2 {
                mode = "GAME";
                level = 2;
                enemies = vec![Vec2::new(255.0, 255.0), Vec2::new(450.0, 450.0), Vec2::new(750.0, 750.0)];
                level2_music.play();
            }
            last_key_2 = key_2_pressed;

            let key_3_pressed = window.is_key_down(Key::Key3);
            if key_3_pressed && !last_key_3 {
                mode = "GAME";
                level = 3;
                enemies = vec![Vec2::new(255.0, 100.0), Vec2::new(450.0, 450.0), Vec2::new(750.0, 750.0)];
            }
            last_key_3 = key_3_pressed;
        }
        if mode == "MENU" {
            render_menu(&mut framebuffer, 1);  
        } else if mode == "SELECT" {
            render_menu(&mut framebuffer, 2); 
        } else if mode == "GAME" {
            framebuffer.set_current_color(Color::new(50,50,100));
            process_event(&window, &mut player, level, block_size, &mut sound_effect, &mut gilrs);
            framebuffer.clear();
            framebuffer.set_current_color(Color::new(50,50,100));
            render3d_with_minimap(&mut framebuffer, &mut player, level, &mut z_buffer, &mut enemies);
            render_enemies(&mut framebuffer, &player, &mut z_buffer, &mut enemies);
        }

        window
            .update_with_buffer(&framebuffer.cast_buffer(), framebuffer_width, framebuffer_height)
            .unwrap();
  
        std::thread::sleep(frame_delay);
    }
}
