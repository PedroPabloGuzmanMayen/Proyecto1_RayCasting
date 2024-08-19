use std::f32::consts::PI;

use nalgebra_glm::{Vec2};
use minifb::{Window, Key};
#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub pos: Vec2,
    pub a: f32,
    pub fov: f32

}

pub fn process_event(window: &Window, player: &mut Player){
    const SPEED:f32 = 5.0;
    const ROTATION_SPEED:f32 = PI/20.0;
    let mut move_x = 0.0;
    let mut move_y = 0.0;
    if window.is_key_down(Key::Left) {
        player.a -= ROTATION_SPEED;
    }
    if window.is_key_down(Key::Right){
        player.a += ROTATION_SPEED;
    }

    if window.is_key_down(Key::Up){
        player.pos.x = player.pos.x + SPEED * player.a.cos();
        player.pos.y = player.pos.y + SPEED * player.a.sin();
        
    
    }

    if window.is_key_down(Key::Down){
        player.pos.x = player.pos.x - SPEED * player.a.cos();
        player.pos.y = player.pos.y - SPEED * player.a.sin();

    }
}