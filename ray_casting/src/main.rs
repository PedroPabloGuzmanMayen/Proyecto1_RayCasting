mod framebuffer;
mod color;
mod bmp;
mod maze;
mod player;
mod caster;
mod render;
mod texture;
use crate::texture::Texture;
use crate::maze::{load_maze};
use crate::framebuffer::FrameBuffer;
use crate::color::Color;
use crate::caster::{cast_ray, Intersect};
use minifb::{Window, WindowOptions, Key};
use crate::player::{Player, process_event};
use std::time::Duration;
use nalgebra_glm::{Vec2};
use render::{render3d_with_minimap, render_menu};



fn main() {
    let window_width = 1300;
    let window_height = 900;
    let block_size = 100;
    let maze = load_maze("assets/levels/level1.txt");
  
    let framebuffer_width = 1300;
    let framebuffer_height = 900;
  
    let frame_delay = Duration::from_millis(15);
  
    let mut framebuffer = framebuffer::FrameBuffer::new(framebuffer_width, framebuffer_height);
    framebuffer.set_current_color(Color::new(50,50,100));

    
  
    let mut window = Window::new(
        "Rust Graphics - Render Loop Example",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();
    let mut player = Player {
        pos: Vec2::new(250.0, 150.0),
        a: std::f32::consts::PI/3.0,
        fov: std::f32::consts::PI/3.0
    };

    let mut mode = "MENU";


    while window.is_open() {
        if window.is_key_down(Key::Escape){
            break;
        }
        if window.is_key_down(Key::A) && mode == "MENU"{
            mode = "GAME"
        }
        if mode == "MENU"{
            render_menu(&mut framebuffer)
        }
        else{
            framebuffer.set_current_color(Color::new(50,50,100));
            process_event(&window, &mut player, &maze, block_size);
            framebuffer.set_current_color(Color::new(50,50,100));
            framebuffer.clear();
            framebuffer.set_current_color(Color::new(50,50,100));
            render3d_with_minimap(&mut framebuffer, &mut player);
        }

        window
            .update_with_buffer(&framebuffer.cast_buffer(), framebuffer_width, framebuffer_height)
            .unwrap();
  
        std::thread::sleep(frame_delay);
    }
  }
  
