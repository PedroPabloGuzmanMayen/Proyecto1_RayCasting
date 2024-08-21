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
use crate::player::{Player, process_event};
use std::time::Duration;
use nalgebra_glm::{Vec2};

fn cell_to_color(cell: char) -> Color {
    match cell {
        '+' => Color::new(0, 255, 0),
        '-' => Color::new(255, 255, 0),
        '|' => Color::new(255, 165, 0),
        ' ' => Color::new(255, 255, 255),
        _ => Color::new(0,0,0)
    }
}

fn draw_cell(framebuffer: &mut FrameBuffer, xo: usize, yo: usize, block_size: usize, cell: char){
    for x in xo..xo+block_size{
        for y in yo..yo+block_size{

            if cell != ' ' {
                let color = cell_to_color(cell);
                framebuffer.set_current_color(color);
                framebuffer.point(x,y);
            }
        }
    }
}


fn render2d(
    framebuffer: &mut FrameBuffer,
    player: &Player,
    offset_x: usize,
    offset_y: usize,
    scale: f32,
) {
    let maze = load_maze("maze.txt");
    let block_size = (100.0 * scale) as usize;

    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            let xo = offset_x + (col * block_size);
            let yo = offset_y + (row * block_size);
            draw_cell(framebuffer, xo, yo, block_size, maze[row][col]);
        }
    }

    framebuffer.set_current_color(Color::new(255, 0, 0));
    let player_x = (player.pos.x * scale) as usize + offset_x;
    let player_y = (player.pos.y * scale) as usize + offset_y;
    framebuffer.point(player_x, player_y);

    let num_rays = 50;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, &maze, &player, a, block_size, true);
    }
}


fn render3d(framebuffer: &mut FrameBuffer, player: &Player) {
    let maze = load_maze("maze.txt");
    let block_size = 100;
    let num_rays = framebuffer.width;

    for i in 0..framebuffer.width {
        for j in 0..(framebuffer.height as f32 / 2.0) as usize {
            framebuffer.set_current_color(Color::new(0, 0, 0));
            framebuffer.point(i, j);
        }
        framebuffer.set_current_color(Color::new(135, 206, 235));
        for j in (framebuffer.height / 2)..framebuffer.height {
            framebuffer.point(i, j);
        }
    }

    let hh = framebuffer.height as f32 / 2.0;
    framebuffer.set_current_color(Color::new(255, 0, 0));
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

        let distance_to_wall = intersect.distance.max(0.1);
        let distance_to_projection_plane = 50.0;
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;
        let stake_top = (hh - (stake_height / 2.0)).max(0.0) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)).min(framebuffer.height as f32 - 1.0) as usize;

        for y in stake_top..stake_bottom {
            let color = cell_to_color(intersect.impact);
            framebuffer.set_current_color(color);
            framebuffer.point(i, y);
        }
    }
}

fn render3d_with_minimap(framebuffer: &mut FrameBuffer, player: &Player) {
    render3d(framebuffer, player); 
    let minimap_scale = 0.2; 
    let minimap_width = (framebuffer.width as f32 * minimap_scale) as usize;
    let minimap_height = (framebuffer.height as f32 * minimap_scale) as usize;
    let minimap_x_offset = framebuffer.width - minimap_width - 10; 
    let minimap_y_offset = 10;
    
    render2d(
        framebuffer,
        player,
        minimap_x_offset,
        minimap_y_offset,
        minimap_scale,
    ); 
}


fn main() {
    let window_width = 1300;
    let window_height = 900;
    let block_size = 100;
    let maze = load_maze("maze.txt");
  
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

    let mut mode = "2D";

    while window.is_open() {
        if window.is_key_down(Key::Escape){
            break;
        }

        if window.is_key_down(Key::Enter){
            if mode == "2D"{
                mode = "3D";
            }
            else {
                mode = "2D";
            }
        }
        framebuffer.set_current_color(Color::new(50,50,100));
        process_event(&window, &mut player, &maze, block_size);
        framebuffer.set_current_color(Color::new(50,50,100));
        framebuffer.clear();
        framebuffer.set_current_color(Color::new(50,50,100));
        render3d_with_minimap(&mut framebuffer, &mut player);

        window
            .update_with_buffer(&framebuffer.cast_buffer(), framebuffer_width, framebuffer_height)
            .unwrap();
  
        std::thread::sleep(frame_delay);
    }
  }
  
