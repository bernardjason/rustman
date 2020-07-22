//use crate::maze_maker::{SQUARE_SIZE, MAZE_HEIGHT};

use crate::rustman::maze_maker::{SQUARE_SIZE, MAZE_HEIGHT};
use crate::rustman::sound::{play};
use crate::rustman::sound;

pub const WIDTH: u32 = 24;
pub const HEIGHT: u32 = 24;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub x_dir: i32,
    pub y_dir: i32,
    pub lives: i32,
    pub score: i32,
    inertia_x: i32,
    inertia_y: i32,
    pub super_pill:i32,
    pub direction: usize,
    old_direction: usize,
    change_direction_back: usize,
    //pub texture: Option<Texture<'a>>,
}



const UP: usize = 2;
const DOWN: usize = 3;
const LEFT: usize = 0;
const RIGHT: usize = 1;
const EAT: usize = 4;

pub const SUPER_PILL_TIME: i32 = 1000;

impl Player {
    pub fn new() -> Player {
        Player {
            lives: 3,
            x: SQUARE_SIZE as i32 * 3 - 2,
            y: (MAZE_HEIGHT as i32 - SQUARE_SIZE as i32 * 3 + 3) as i32,
            x_dir: 0,
            y_dir: 0,
            score: 0,
            inertia_x: 0,
            inertia_y: 0,
            direction: 0,
            old_direction: 0,
            super_pill:0,
            change_direction_back: 0,
        }
    }
    pub fn player_reset(&mut self) {
        self.x =SQUARE_SIZE as i32 * 3 - 2 ;
        self.y =(MAZE_HEIGHT as i32 - SQUARE_SIZE as i32 * 3 + 3) as i32 ;
        self.x_dir=0;
        self.y_dir=0;
        self.inertia_y = 0;
        self.inertia_x = 0;

    }

    pub fn render(&mut self, maze: &mut Vec<Vec<char>>) -> (bool, i32, i32) {
        let mut stale = false;
        self.set_inertia(maze);

        self.apply_inertia(maze);

        if self.super_pill > 0 {
            self.super_pill = self.super_pill -1;
        }

        let (eaten, x, y) = self.check_and_eat_pill(maze);
        if eaten {
            self.score = self.score + 1;
            play(sound::CHOMP);
            stale = true;
        }
        if self.change_direction_back > 0 && self.direction == EAT {
            self.change_direction_back = self.change_direction_back - 1;
            if self.change_direction_back == 0 {
                self.direction = self.old_direction;
            }
        }
        return (stale, x, y);
    }


    fn set_inertia(&mut self, maze: &Vec<Vec<char>>) {
        if self.x_dir != 0 {
            let new_x = self.x + self.x_dir;
            if self.check_if_move_ok(maze, new_x, self.y) {
                self.inertia_x = self.x_dir;
            }
        }
        if self.y_dir != 0 {
            let new_y = self.y + self.y_dir;
            if self.check_if_move_ok(maze, self.x, new_y) {
                self.inertia_y = self.y_dir;
            }
        }
    }

    fn apply_inertia(&mut self, maze: &Vec<Vec<char>>) {
        if self.inertia_x != 0 {
            let new_x = self.x + self.inertia_x * 2;
            if self.check_if_move_ok(maze, new_x, self.y) {
                if self.change_direction_back == 0 {
                    if self.inertia_x < 0 {
                        self.direction = LEFT;
                    } else {
                        self.direction = RIGHT;
                    }
                }
                self.x = self.x + self.inertia_x;
                if self.x_dir != 0 {
                    self.inertia_y = 0;
                }
                self.x_dir = 0;
            } else {
                self.inertia_x = 0;
            }
        }

        if self.inertia_y != 0 {
            let new_y = self.y + self.inertia_y * 2;
            if self.check_if_move_ok(maze, self.x, new_y) {
                self.y = self.y + self.inertia_y;
                if self.change_direction_back == 0 {
                    if self.inertia_y < 0 {
                        self.direction = UP;
                    } else {
                        self.direction = DOWN;
                    }
                }
                if self.y_dir != 0 {
                    self.inertia_x = 0;
                }
                self.y_dir = 0;
            } else {
                self.inertia_y = 0;
            }
        }
    }
    fn check_and_eat_pill(&mut self, maze: &mut Vec<Vec<char>>) -> (bool, i32, i32) {
        let centre_x = self.x;
        let centre_y = self.y - SQUARE_SIZE as i32 * 3;
        let what = maze[centre_y as usize / SQUARE_SIZE][centre_x as usize / SQUARE_SIZE];
        if what == '*' || what == 'P' {
            maze[centre_y as usize / SQUARE_SIZE][centre_x as usize / SQUARE_SIZE] = ' ';
            let wherex = (centre_x / SQUARE_SIZE as i32) * SQUARE_SIZE as i32;
            let wherey = (centre_y / SQUARE_SIZE as i32) * SQUARE_SIZE as i32;
            self.old_direction = self.direction;
            self.direction = EAT;
            self.change_direction_back = 10;
            if  what == 'P'  {
                self.super_pill = SUPER_PILL_TIME;
            }
            (true, wherex, wherey)
        } else {
            (false, 0, 0)
        }
    }

    fn check_if_move_ok(&mut self, maze: &Vec<Vec<char>>, new_x: i32, new_y: i32) -> bool {
        let half_square = SQUARE_SIZE as i32 / 2;
        let centre_x = new_x + half_square;
        let centre_y = new_y - half_square;

        let xarray = [centre_x - 17 as i32, centre_x, centre_x + 4];
        let yarray = [centre_y - 31, centre_y - 24, centre_y - 19, centre_y - 12, centre_y - 9];
        let mut all_okay = true;
        for y_cell in yarray.iter() {
            for x_cell in xarray.iter() {
                let c = maze[*y_cell as usize / SQUARE_SIZE][*x_cell as usize / SQUARE_SIZE];
                if c == 'X' || c == '!'  {
                    all_okay = false;
                }
            }
        }
        return all_okay;
    }
}
