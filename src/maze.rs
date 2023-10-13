use core::fmt;
use std::ops::Range;
use rand::{seq::SliceRandom, rngs::ThreadRng, Rng};

use bevy::{prelude::*, window::PrimaryWindow};


const WALL_THICKNESS: f32 = 5.;
const WALL_LENGTH: f32 = 50.;
const WALL_COLOR: Color = Color::rgb(0.4, 0.4, 0.4);
const WALL_VERTICAL_RAND: Range<usize> = 5..11;
const WALL_HORIZONTAL_RAND: Range<usize> = 5..11;


#[derive(Component)]
pub struct Wall;

#[derive(Bundle)]
pub struct WallBundle {
    sprite_bundle: SpriteBundle,
    wall: Wall
}

enum Toward {
    Horizontal,
    Vertical
}

enum Direction {
    Up,
    Left,
    Right,
    Down
}


#[derive(Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn update(self, direction: &Direction, step: usize) -> Result<Position, String> {
        let mut new = self;
        match direction {
            Direction::Up => new.y += step,
            Direction::Left => {
                if new.x < step {
                    return Err("attempt to subtract with overflow".to_string())
                }
                new.x -= step
            }
            Direction::Right => new.x += step,
            Direction::Down => {
                if new.y < step {
                    return Err("attempt to subtract with overflow".to_string())
                }
                new.y -= step
            }
        }
        Ok(new)
    }
}

// pub struct MazePlugin;

// impl Plugin for MazePlugin {
//     fn build(&self, app: &mut App) {
//         app.add_systems(Update, spawn_maze);
//     }
// }

pub fn spawn_maze(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    let center_x = window.width() / 2.0;
    let center_y = window.height() / 2.0;

    let width = rand_odd_between(WALL_HORIZONTAL_RAND);
    let height = rand_odd_between(WALL_VERTICAL_RAND);

    let maze = Maze::new(width, height);
    
    // horizontal wall
    for row in (0..maze.b_height).step_by(2) {
        for col in (1..maze.b_width-1).step_by(2) {
            if maze.data[row][col] == 1 {
            let x = (((col-1) as f32 - (maze.b_width as f32 / 2.))+1.) * WALL_LENGTH/2.+ center_x;
            let y = ((row as f32 - (maze.b_height as f32 / 2.))-1.) * WALL_LENGTH/2. + center_y;
                commands.spawn(WallBundle::spawn_wall(x, y, Toward::Horizontal));
            }
        }
    }
    // vertical wall
    for row in (1..maze.b_height-1).step_by(2) {
        for col in (0..maze.b_width).step_by(2) {
            if maze.data[row][col] == 1 {
                let x = (col as f32 - (maze.b_width as f32 / 2.)) * WALL_LENGTH/2. + center_x;
                let y = ((row - 1) as f32 - (maze.b_height as f32 / 2.)) * WALL_LENGTH/2. + center_y;
                commands.spawn(WallBundle::spawn_wall(x, y, Toward::Vertical));
            }
        }
    }

}

impl WallBundle {
    fn spawn_wall(x: f32, y: f32, toward: Toward) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle { 
                sprite: Sprite { 
                    color: WALL_COLOR,
                    ..default()
                },
                transform: Transform { 
                    translation: Vec3 { x, y, z: 0. }, 
                    scale: match toward {
                        Toward::Horizontal => Vec3 { x: WALL_LENGTH, y: WALL_THICKNESS, z: 0. },
                        Toward::Vertical => Vec3 { x: WALL_THICKNESS, y: WALL_LENGTH, z: 0. },
                    },
                    ..default()
                },
                ..default()
            },
            wall: Wall
        }
    }
} 



type MazeData = Vec<Vec<u8>>;

#[derive(Component)]
pub struct Maze {
    // length of edge
    pub width: usize,
    pub height: usize,
    b_width: usize,
    b_height: usize,

    data: MazeData,
}

impl fmt::Display for Maze {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new();
        for row in &self.data {
            for ele in row {
                if *ele == 0 {
                    res.push_str("#");
                } else if *ele == 1 {
                    res.push_str("0");
                }
            }
            res.push_str("\n");
        }

        write!(f, "{}", res)
    }
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        let mut maze = Maze {
            width: width,
            height: height,
            b_width: width * 2 + 1,
            b_height: height * 2 + 1,
            data: vec![vec![1; width*2+1]; height*2+1],
        };

        // initialize
        for i in (1..=(maze.b_height-1)).step_by(2) {
            for j in (1..=(maze.b_width-1)).step_by(2) {   
                maze.data[i][j] = 0;
            }
        }

        let mut rng = rand::thread_rng();
        
        maze.iter(Position{x: 1, y: 1}, &mut rng);   
        return maze
    }


    


    fn iter (&mut self, start: Position, rng: &mut ThreadRng) {
        let mut directions = [
            Direction::Up,
            Direction::Left,
            Direction::Right,
            Direction::Down,
        ];
        directions.shuffle(rng); 

        for direction in directions {
            
            let next;
            
            if let Ok(new_pos) = start.update(&direction, 2) {
                next = new_pos;
            } else {
                continue;
            }

            if self.is_position_exceed(next) {
                continue
            }

            if self.has_position_visited(&next) {
                let wall = start;
                if let Ok(wall) = wall.update(&direction, 1) {
                    self.data[wall.y][wall.x] = 0;
                } else {
                    println!("over range")
                }
                
                self.iter(next, rng);
            }
        }
    }

    fn has_position_visited(&self, pos: &Position) -> bool {
        let max_row = self.data.len();
        let max_col = if max_row > 0 { self.data[0].len() } else { 0 };

        if pos.y > 0 && pos.y + 1 < max_row && pos.x > 0 && pos.x + 1 < max_col {
            self.data[pos.y+1][pos.x] == 1
            && self.data[pos.y-1][pos.x] == 1
            && self.data[pos.y][pos.x+1] == 1
            && self.data[pos.y][pos.x-1] == 1
        } else {
            false
        }
    } 
    
    fn is_position_exceed(&self, pos: Position) -> bool {
        pos.x > self.b_width || pos.y > self.b_height
    }
    
}


pub fn rand_odd_between(range: Range<usize>) -> usize {
    let mut rng = rand::thread_rng();
    loop {
        let num = rng.gen_range(range.clone());
        if num % (2) != 0 {
            return num;
        }
    }
}