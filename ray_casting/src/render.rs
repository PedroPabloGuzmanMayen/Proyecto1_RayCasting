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
static WELCOME:Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/textures/Welcome.png")));
static COIN:Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/textures/key_big.png")));



fn cell_to_texture(cell: char, tx:u32, ty:u32, level: usize) -> Color {
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
        ' ' => wall2.get_pixel_color(tx, ty),
        _ => Color::new(0,0,0)
    }
}

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
        cast_ray(framebuffer, &maze, &player, a, block_size, true);
    }
}


fn render3d(framebuffer: &mut FrameBuffer, player: &Player, level:usize, z_Buffer: &mut [f32]) {

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
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

        let distance_to_wall = intersect.distance.max(0.1);
        let distance_to_projection_plane = 50.0;
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;
        let stake_top = (hh - (stake_height / 2.0)).max(0.0) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)).min(framebuffer.height as f32 - 1.0) as usize;
        z_Buffer[i] = distance_to_wall;
        for y in stake_top..stake_bottom {
            let ty = (y as f32-stake_top as f32)/(stake_bottom as f32-stake_top as f32) * 128.0;
            let tx = intersect.tx;
            let color = cell_to_texture(intersect.impact, tx as u32, ty as u32, level );
            framebuffer.set_current_color(color);
            framebuffer.point(i, y);
        }
    }
}

pub fn render3d_with_minimap(framebuffer: &mut FrameBuffer, player: &Player, level:usize, z_Buffer: &mut [f32]) {
    render3d(framebuffer, player, level, z_Buffer); 
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

pub fn render_menu(framebuffer: &mut FrameBuffer) {
    framebuffer.clear();

    let texture_width = 128.0;
    let texture_height = 128.0;

    let fb_width = framebuffer.width;
    let fb_height = framebuffer.height;

    for x in 0..fb_width {
        for y in 0..fb_height {
            
            let tx = (x as f32 / fb_width as f32 * texture_width as f32) as u32;
            let ty = (y as f32 / fb_height as f32 * texture_height as f32) as u32;

            let color = WALL1.get_pixel_color(tx, ty);
            framebuffer.set_current_color(color);
            framebuffer.point(x, y);
        }
    }
}

pub fn render_enemy(framebuffer: &mut FrameBuffer, player: &Player, pos:&Vec2, z_Buffer: &mut [f32]){
    let sprite_a = (pos.y - player.pos.y).atan2(pos.x - player.pos.x);

    if sprite_a < 0.0 {
        return;
    }

    let sprite_d = ((player.pos.x - pos.x).powi(2) + (player.pos.y - pos.y).powi(2)).sqrt();


    if sprite_d < 10.0{
        return;
    }

    let screen_height = framebuffer.height;
    let screen_width = framebuffer.width;

    let sprite_size = (screen_height as f32 / sprite_d) * 100.0;
    let start_x = ((sprite_a - player.a) * (screen_height as f32 / player.fov) + (screen_width as f32 / 2.0) - (sprite_size / 2.0)).max(0.0);
    let start_y = ((screen_height as f32/2.0) - (sprite_size/2.0)).max(0.0);
    let end_x = ((start_x + sprite_size) as usize).min(framebuffer.width);
    let end_y = ((start_y+ sprite_size) as usize).min(framebuffer.height);

    if end_x <= 0{
        return;
    }

    if (start_x as usize) < framebuffer.width && sprite_d < z_Buffer[start_x as usize ]{
        for x in start_x as usize..end_x{
            for y in start_y as usize..end_y as usize{
                let tx = ((x -start_x as usize) * 32 / sprite_size as usize) as u32;
                let ty = (((y - start_y as usize)) * 32 / sprite_size as usize) as u32;
                let color = COIN.get_pixel_color(tx, ty);
                if color.to_hex() != Color::new(0,0,0).to_hex() && color.to_hex() != Color::new(255,0,0).to_hex(){
                    framebuffer.set_current_color(color);
                    framebuffer.point(x,y)
                }
            }
    
            z_Buffer[x] = sprite_d;
        }
    }
    
    
}
pub fn render_enemies(framebuffer: &mut FrameBuffer, player: &Player, z_Buffer: &mut [f32]){
    let enemies = vec![Vec2::new(350.0, 350.0)];

    for enemy in enemies{
        render_enemy(framebuffer, player, &enemy, z_Buffer);
    }
}

