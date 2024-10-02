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


pub fn cast_maze(loaded_maze: &mut Vec<Vec<char>>, enemies: &mut Vec<Vec2>, winning_position: &mut [usize], block_size: usize) -> Vec<Vec<char>>{

    let mut new_maze: Vec<Vec<char>> = loaded_maze.clone();
    for i in 0..(loaded_maze.len()) {
        for j in 0..(loaded_maze[i].len()){
            if loaded_maze[i][j] == 'e'{
                new_maze[i][j] = ' ';
                enemies.push(Vec2::new((i as usize * block_size) as f32, (j as usize * block_size) as f32));
            }
            if loaded_maze[i][j] == 'w'{
                new_maze[i][j] = ' ';
            }
        }
        

       
    }
    new_maze
}

