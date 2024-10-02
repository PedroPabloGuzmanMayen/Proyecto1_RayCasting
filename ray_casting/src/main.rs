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


fn main() {
    let window_width = 1300;
    let window_height = 900;
    let block_size = 100;
    let maze = load_maze("assets/levels/level1.txt");

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
    let mut enemies: Vec<Vec2> = vec![];
    //let audio_player = AudioPlayer::new("assets/music/Jungle.mp3");


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
        if window.is_key_down(Key::Key1) && mode == "MENU"{
            mode = "GAME";
            level = 1;
            enemies = vec![Vec2::new(255.0, 255.0), Vec2::new(450.0, 450.0), Vec2::new(750.0, 750.0)];
            //audio_player.play();
        }
        if window.is_key_down(Key::Key2) && mode == "MENU"{
            mode = "GAME";
            level = 2;
            enemies = vec![Vec2::new(255.0, 255.0), Vec2::new(450.0, 450.0), Vec2::new(750.0, 750.0)]
        }
        if window.is_key_down(Key::Key3) && mode == "MENU"{
            mode = "GAME";
            level = 3;
            enemies = vec![Vec2::new(255.0, 100.0), Vec2::new(450.0, 450.0), Vec2::new(750.0, 750.0)]
        }
        if mode == "MENU"{
            render_menu(&mut framebuffer)
        }
        else{
            framebuffer.set_current_color(Color::new(50,50,100));
            process_event(&window, &mut player, level, block_size);
            framebuffer.set_current_color(Color::new(50,50,100));
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
  
