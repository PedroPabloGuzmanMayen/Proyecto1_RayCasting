mod framebuffer;
mod color;
mod bmp;
mod maze;
mod player;
mod caster;
use crate::maze::{load_maze};
use crate::framebuffer::FrameBuffer;
use crate::color::Color;
use crate::caster::{cast_ray, Intersect};
use minifb::{Window, WindowOptions, Key};
use crate::player::Player;
use std::time::Duration;
use nalgebra_glm::{Vec2};
fn draw_cell(framebuffer: &mut FrameBuffer, xo: usize, yo: usize, block_size: usize, cell: char){
    for x in xo..xo+block_size{
        for y in yo..yo+block_size{

            if cell != ' ' {
                framebuffer.point(x,y);
            }
        }
    }
}
fn render2d(framebuffer: &mut FrameBuffer, player: &mut Player){
    let maze = load_maze("maze.txt");
    let block_size = 100;

    for row in 0..maze.len(){
        for col in 0..maze[row].len(){
            draw_cell(framebuffer, col*block_size, row*block_size, block_size, maze[row][col]);
        }
    }
    framebuffer.set_current_color(Color::new(255,0,0));
    framebuffer.point(player.pos.x as usize, player.pos.y as usize);
    cast_ray(framebuffer, &maze, &player, player.a, block_size, true);
}
fn main() {
    let window_width = 1300;
    let window_height = 900;
  
    let framebuffer_width = 1300;
    let framebuffer_height = 900;
  
    let frame_delay = Duration::from_millis(16);
  
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
        a: std::f32::consts::PI/3.0
    };

  
    while window.is_open() {
        framebuffer.set_current_color(Color::new(50,50,100));
        render2d(&mut framebuffer, &mut player);
  
        
  
        window
            .update_with_buffer(&framebuffer.cast_buffer(), framebuffer_width, framebuffer_height)
            .unwrap();
  
        std::thread::sleep(frame_delay);
    }
  }
  
