use crate::maze::{load_maze};
use crate::framebuffer::FrameBuffer;
use crate::color::Color;
use crate::player::Player;
use crate::texture::Texture;
use nalgebra_glm::{Vec2};
use crate::caster::{cast_ray, Intersect};
use minifb::{Window, Key};
use once_cell::sync::Lazy;
use std::sync::Arc;


static WALL1:Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/textures/wall1.png")));
static WALL2:Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/textures/wall2.png")));
static WALL3:Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/textures/computer.png")));
static WALL4:Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/textures/ventilador.png")));
static WALL5:Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/textures/wall4.png")));
static WALL6:Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/textures/wall3.png")));
static WELCOME:Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/textures/main.png")));
static COIN:Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/textures/key_big.png")));
static SELECT:Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/textures/select.png")));
static END:Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/textures/end.png")));

fn cell_to_texture(cell: char, tx:u32, ty:u32, level: usize, enemies: &mut Vec<Vec2>) -> Color {
    let wall = match level {
        1 => WALL1.clone(),
        2 => WALL3.clone(),
        3 => WALL5.clone(),
        _ => WALL1.clone()
    };

    let wall2 = match level {
        1 => WALL2.clone(),
        2 => WALL4.clone(),
        3 => WALL6.clone(),
        _ => WALL2.clone()
    };
    match cell {
        '+' => wall.get_pixel_color(tx, ty),
        '-' => wall2.get_pixel_color(tx, ty),
        '|' => wall.get_pixel_color(tx, ty),
        'w' => Color::new(255,0,0),
        _ => Color::new(0,0,0)
    }
}

fn cell_to_color(cell: char) -> Color {
    match cell {
        '+' => Color::new(0, 255, 0),
        '-' => Color::new(255, 255, 0),
        '|' => Color::new(255, 165, 0),
        ' ' => Color::new(255, 255, 255),
        'w' => Color::new(255,0,0),
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
    level:usize
) {

    let level_name = match level {
        1 => "assets/levels/level1.txt",
        2 => "assets/levels/level2.txt",
        3 => "assets/levels/level3.txt",
        _ => "assets/levels/level1.txt"
    };
    let maze = load_maze(level_name);
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

    let num_rays = 5;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, &maze, &player, a, block_size, true, 0.7);
    }
}


fn render3d(framebuffer: &mut FrameBuffer, player: &Player, level:usize, z_Buffer: &mut [f32], enemies: &mut Vec<Vec2>) {

    let level_name = match level {
        1 => "assets/levels/level1.txt",
        2 => "assets/levels/level2.txt",
        3 => "assets/levels/level3.txt",
        _ => "assets/levels/level1.txt"
    };
    let maze = load_maze(level_name);
    let block_size = 100;
    let num_rays = framebuffer.width;

    for i in 0..framebuffer.width {
        for j in 0..(framebuffer.height as f32 / 2.0) as usize {
            framebuffer.set_current_color(Color::new(0, 0, 0));
            framebuffer.point(i, j);
        }
        framebuffer.set_current_color(Color::new(128, 128, 128));
        for j in (framebuffer.height / 2)..framebuffer.height {
            framebuffer.point(i, j);
        }
    }

    let hh = framebuffer.height as f32 / 2.0;
    framebuffer.set_current_color(Color::new(255, 0, 0));
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false, 1.0);

        let distance_to_wall = intersect.distance.max(0.1);
        let distance_to_projection_plane = 50.0;
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;
        let stake_top = (hh - (stake_height / 2.0)).max(0.0) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)).min(framebuffer.height as f32 - 1.0) as usize;
        z_Buffer[i] = distance_to_wall;
        for y in stake_top..stake_bottom {
            let ty = (y as f32-stake_top as f32)/(stake_bottom as f32-stake_top as f32) * 128.0;
            let tx = intersect.tx;
            let color = cell_to_texture(intersect.impact, tx as u32, ty as u32, level, enemies);
            framebuffer.set_current_color(color);
            framebuffer.point(i, y);
        }
    }
}

//Minimapa (10 puntos)
pub fn render3d_with_minimap(framebuffer: &mut FrameBuffer, player: &Player, level:usize, z_Buffer: &mut [f32], enemies: &mut Vec<Vec2>) {
    render3d(framebuffer, player, level, z_Buffer,enemies); 
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
        level
    ); 
}
//Pantalla de bienvenida (5 puntos)
pub fn render_menu(framebuffer: &mut FrameBuffer, mode: usize) {
    framebuffer.clear();

    let image = match mode {
        1 => WELCOME.clone(),
        2 => SELECT.clone(),
        _ => END.clone()
    };

    let texture_width = 1920.0;
    let texture_height = 1080.0;

    let fb_width = framebuffer.width;
    let fb_height = framebuffer.height;

    for x in 0..fb_width {
        for y in 0..fb_height {
            
            let tx = (x as f32 / fb_width as f32 * texture_width as f32) as u32;
            let ty = (y as f32 / fb_height as f32 * texture_height as f32) as u32;

            let color = image.get_pixel_color(tx, ty);
            framebuffer.set_current_color(color);
            framebuffer.point(x, y);
        }
    }
}


