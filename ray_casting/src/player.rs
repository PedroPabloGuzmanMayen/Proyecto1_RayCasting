use std::f32::consts::PI;
use crate::maze::{load_maze};
use nalgebra_glm::{Vec2};
use minifb::{Window, Key};
#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub pos: Vec2,
    pub a: f32,
    pub fov: f32

}

pub fn process_event(window: &Window, player: &mut Player, level: usize, block_size: usize) {
    const SPEED: f32 = 25.0;
    const ROTATION_SPEED: f32 = PI / 20.0;

    let lvl_name = match level {
        1 => "assets/levels/level1.txt",
        2 => "assets/levels/level2.txt",
        3 => "assets/levels/level3.txt",
        _ => "assets/levels/level1.txt"
    };

    let maze = load_maze(lvl_name);

    if window.is_key_down(Key::Left) {
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(Key::Right) {
        player.a += ROTATION_SPEED;
    }

    let mut next_x;
    let mut next_y;

    if window.is_key_down(Key::Up) {
        next_x = player.pos.x + SPEED * player.a.cos();
        next_y = player.pos.y + SPEED * player.a.sin();
        if !is_wall(&maze, next_x, next_y, block_size) {
            player.pos.x = next_x;
            player.pos.y = next_y;
        }
    }

    if window.is_key_down(Key::Down) {
        next_x = player.pos.x - SPEED * player.a.cos();
        next_y = player.pos.y - SPEED * player.a.sin();
        if !is_wall(&maze, next_x, next_y, block_size) {
            player.pos.x = next_x;
            player.pos.y = next_y;
        }
    }
}



fn is_wall(maze: &Vec<Vec<char>>, x: f32, y: f32, block_size: usize) -> bool {
    let row = (y / block_size as f32) as usize;
    let col = (x / block_size as f32) as usize;

    if row >= maze.len() || col >= maze[row].len() {
        return false;
    }

    maze[row][col] != ' '
}
