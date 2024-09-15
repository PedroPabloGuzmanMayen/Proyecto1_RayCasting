use crate::framebuffer::FrameBuffer;
use crate::player::Player;
use crate::color::Color;


pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub tx: usize
}

pub fn cast_ray(framebuffer: &mut FrameBuffer, maze: &Vec<Vec<char>>, player: &Player,
    a: f32, block_size: usize, draw_line: bool) -> Intersect {
let mut d = 0.0;
let mut x;
let mut y;

framebuffer.set_current_color(Color::new(255, 0, 0));

loop {

let cos = a.cos();
let sin = a.sin();
x = (player.pos.x + d * cos) as usize;
y = (player.pos.y + d * sin) as usize;


let i = x / block_size;
let j = y / block_size;
let tx = x- i * block_size;
let hitx = y-j * block_size;
let hity = x-i * block_size;
let mut max_hit = hity;

if 1 < hitx &&  hitx < block_size-1{
    max_hit = hitx
}
if j >= maze.len() || i >= maze[j].len() {
    break;  
}

if draw_line {
 framebuffer.point(x, y);  
}

if maze[j][i] != ' ' {
return Intersect {
    distance: d,
    impact: maze[j][i],
    tx: max_hit
};
}

d += 0.1; 
}
Intersect {
    distance: d,
    impact: ' ',
    tx: 0
}
}
