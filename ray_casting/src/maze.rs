use std::fs::File;
use std::io::{BufRead, BufReader};
use nalgebra_glm::{Vec2};
pub fn load_maze(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

pub fn get_win_position(loaded_maze: &mut Vec<Vec<char>>, block_size: usize) -> Vec2{
    let mut x = 0.0;
    let mut y = 0.0;
    for i in 0..(loaded_maze.len()) {
        for j in 0..(loaded_maze[i].len()){
            if loaded_maze[i][j] == 'w'{
                x = j as f32 * block_size as f32;
                y = i as f32 * block_size as f32;
            }
        }
        

       
    }

    Vec2::new(x, y)
}



