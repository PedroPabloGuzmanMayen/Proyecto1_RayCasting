use std::f32::consts::PI;
use crate::maze::{load_maze};
use nalgebra_glm::{Vec2};
use minifb::{Window, Key};
use crate::music::AudioPlayer;
use gilrs::{Gilrs, Button, Event};
#[derive(Debug, Copy, Clone)]

pub struct Player {
    pub pos: Vec2,
    pub a: f32,
    pub fov: f32,
    pub mouse_sensitivity: f32,
    pub last_mouse_x: f32

}
// Camera con movimiento (10 puntos)
pub fn process_event(window: &Window, player: &mut Player, level: usize, block_size: usize, gilrs: &mut Gilrs) {
    const SPEED: f32 = 8.0;
    const ROTATION_SPEED: f32 = PI / 60.0;

    let lvl_name = match level {
        1 => "assets/levels/level1.txt",
        2 => "assets/levels/level2.txt",
        3 => "assets/levels/level3.txt",
        _ => "assets/levels/level1.txt"
    };

    let maze = load_maze(lvl_name);
    let moved = false;

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
            println!("Player x position {}, player y postion {}", player.pos.x, player.pos.y);
        }
    }

    if window.is_key_down(Key::Down) {
        next_x = player.pos.x - SPEED * player.a.cos();
        next_y = player.pos.y - SPEED * player.a.sin();
        if !is_wall(&maze, next_x, next_y, block_size) {
            player.pos.x = next_x;
            player.pos.y = next_y;
            println!("Player x position {}, player y postion {}", player.pos.x, player.pos.y);
        }

    }

    //Soporte para rotación con mouse (1o puntos)
    if let Some(mouse_pos) = window.get_mouse_pos(minifb::MouseMode::Discard) {
        let mouse_x = mouse_pos.0 as f32;
        let mouse_sensitivity = player.mouse_sensitivity;

        let delta_x = mouse_x - player.last_mouse_x;

        player.a -= delta_x * mouse_sensitivity;
        player.last_mouse_x = mouse_x;

        player.a = player.a % (2.0 * PI);
    }

    //Soporte de control (20 puntos)
    while let Some(Event { id, event, .. }) = gilrs.next_event() {
        match event {
            gilrs::EventType::ButtonPressed(Button::DPadLeft, _) => {
                player.a -= ROTATION_SPEED;
            }
            gilrs::EventType::ButtonPressed(Button::DPadRight, _) => {
                player.a += ROTATION_SPEED;
            }
            gilrs::EventType::ButtonPressed(Button::DPadUp, _) => {
                next_x = player.pos.x + SPEED * player.a.cos();
                next_y = player.pos.y + SPEED * player.a.sin();
                if !is_wall(&maze, next_x, next_y, block_size) {
                    player.pos.x = next_x;
                    player.pos.y = next_y;
                }
            }
            gilrs::EventType::ButtonPressed(Button::DPadDown, _) => {
                next_x = player.pos.x - SPEED * player.a.cos();
                next_y = player.pos.y - SPEED * player.a.sin();
                if !is_wall(&maze, next_x, next_y, block_size) {
                    player.pos.x = next_x;
                    player.pos.y = next_y;
                }
            }
            _ => {}
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

