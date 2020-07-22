use std::f64;

extern crate pretty_env_logger;

use log::{info, debug};

extern crate rand; // 0.6.5

use rand::Rng;
use crate::rustman::maze_maker::{SQUARE_SIZE, xy_to_cell, MAZE_WIDTH, MAZE_HEIGHT, MAZE_X, MAZE_Y};


pub const WIDTH: u32 = 24;
pub const HEIGHT: u32 = 24;


pub struct Ghost {
    pub x: i32,
    pub y: i32,
    pub x_dir: i32,
    pub y_dir: i32,
    inertia_x: i32,
    inertia_y: i32,
    pub target: (i32, i32),
    pub current_target: (i32, i32),
    clicks: i32,
    smart_seconds: i32,
    startx: i32,
    starty: i32,
    pub(crate) stop: bool,
    out_of_home: bool,
}

pub fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {
    return (((x2 - x1) as f64).powf(2_f64) + ((y2 - y1) as f64).powf(2_f64)).sqrt();
}

impl Ghost {
    pub fn new(x_dir: i32, y_dir: i32, x: i32, y: i32, target_x: i32, target_y: i32) -> Ghost {
        Ghost {
            x: x,
            y: y,
            x_dir: x_dir,
            y_dir: y_dir,
            inertia_x: 0,
            inertia_y: 0,
            target: (x, y),
            current_target: (target_x, target_y),
            clicks: 0,
            smart_seconds: 0,
            //id: get_id(),
            startx: x,
            starty: y,
            stop: false,
            out_of_home: false,
        }
    }
    pub fn ghost_reset(&mut self) {
        self.x = self.startx;
        self.y = self.starty;
        self.x_dir = 0; self.inertia_x = 0;
        self.y_dir = -1 ; self.inertia_y = -1;
    }

    pub fn render(&mut self, maze: &mut Vec<Vec<char>>) {
        self.clicks = self.clicks + 1;
        if self.clicks % 60 == 0 {
            self.smart_seconds = self.smart_seconds + 1;
        }
        if self.clicks % 240 == 0 {
            self.out_of_home = true;
        }

        if self.out_of_home {
            self.set_current_target_to_latest_target();
        }

        self.set_inertia(maze);

        let away = self.update_current_target_if_reached();

        let (player_cellx, player_celly) = xy_to_cell(self.target.0 + 2, self.target.1 - SQUARE_SIZE as i32 * 2);
        let (ghost_cellx, ghost_celly) = xy_to_cell(self.x + 4, self.y - 16);

        self.debug_if_stopped(away, player_cellx, player_celly, ghost_cellx, ghost_celly);

        self.consider_chasing_player_if_directly_visible(maze, player_cellx, player_celly, ghost_cellx, ghost_celly);

        if !self.stop {
            self.apply_inertia(maze);
        }
    }

    pub fn change_direction(&mut self, maze: &mut Vec<Vec<char>>) {
        debug!("{} change_direction inertia_x{} x_dir{} inertia_y{} y_dir{}", self.clicks, self.inertia_x, self.x_dir, self.inertia_y, self.y_dir);
        if self.inertia_x != 0 && self.inertia_x == self.x_dir && self.inertia_y == 0 {
            self.x_dir = self.x_dir * -1; // tried one way try other
        } else if self.inertia_y != 0 && self.inertia_y == self.y_dir && self.inertia_x == 0 {
            self.y_dir = self.y_dir * -1; // tried one way try other
        } else {
            if self.inertia_x != 0 {
                self.y_dir = self.up_down_better(maze);
                self.x_dir = 0;
                self.inertia_x = 0;
            } else if self.inertia_y != 0 {
                self.x_dir = self.left_right_better(maze);
                self.y_dir = 0;
                self.inertia_y = 0;
            } else {
                self.x_dir = rand::thread_rng().gen_range(-1, 1);
                self.y_dir = rand::thread_rng().gen_range(-1, 1);
            }
        }
        debug!("{} end change_direction inertia_x {} x_dir{} inertia_y{} y_dir{}", self.clicks, self.inertia_x, self.x_dir, self.inertia_y, self.y_dir);
    }


    fn left_right_better(&mut self, maze: &mut Vec<Vec<char>>) -> i32 {
        let left = distance(self.x - SQUARE_SIZE as i32 * 3, self.y, self.current_target.0, self.current_target.1);
        let right = distance(self.x + SQUARE_SIZE as i32 * 3, self.y, self.current_target.0, self.current_target.1);
        let leftok = self.check_if_move_ok(maze, self.x - 1, self.y);
        let rightok = self.check_if_move_ok(maze, self.x + 2, self.y);
        if leftok && left < right {
            debug!("{} -1 leftright left{}{} right{}{}", self.clicks, left, leftok, right, rightok);
            return -1;
        } else if rightok && right < left {
            debug!("{} 1 leftright left{}{} right{}{}", self.clicks, left, leftok, right, rightok);
            return 1;
        }
        if leftok {
            debug!("{} ONLY LEFT -1 leftright left{}{} right{}{}", self.clicks, left, leftok, right, rightok);
            return -1;
        }
        if rightok {
            debug!("{} ONLY RIGHT 1 leftright left{}{} right{}{}", self.clicks, left, leftok, right, rightok);
            return 1;
        }
        debug!("{} 0 leftright left{}{} right{}{}", self.clicks, left, leftok, right, rightok);
        return 0;
    }

    fn up_down_better(&mut self, maze: &mut Vec<Vec<char>>) -> i32 {
        let down = distance(self.x, self.y + SQUARE_SIZE as i32 * 3, self.current_target.0, self.current_target.1);
        let up = distance(self.x, self.y - SQUARE_SIZE as i32 * 3, self.current_target.0, self.current_target.1);
        let upok = self.check_if_move_ok(maze, self.x, self.y - 1);
        let downok = self.check_if_move_ok(maze, self.x, self.y + 1);
        if upok && up < down {
            debug!("{} -1 updown up{}{} down{}{}", self.clicks, up, upok, down, downok);
            return -1;
        } else if downok && down < up {
            debug!("{} 1 updown up{}{} down{}{}", self.clicks, up, upok, down, downok);
            return 1;
        }
        if upok {
            debug!("{} ONLY UP 1 updown up{}{} down{}{}", self.clicks, up, upok, down, downok);
            return -1;
        }
        if downok {
            debug!("{} ONLY DOWN 1 updown up{}{} down{}{}", self.clicks, up, upok, down, downok);
            return 1;
        }
        debug!("{} 0 updown up{}{} down{}{}", self.clicks, up, upok, down, downok);
        return 0;
    }


    fn debug_if_stopped(&mut self, away: f64, player_cellx: i32, player_celly: i32, ghost_cellx: i32, ghost_celly: i32) {
        if self.stop && self.clicks % 240 == 0 {
            let (xcell, ycell) = xy_to_cell(self.x, self.y);
            info!("DEBUG!!!!  player={},{} ghost={},{}", player_cellx, player_celly, ghost_cellx, ghost_celly);
            info!(" AWAY {}  CURRENT_TARGET {},{} ", away, self.current_target.0, self.current_target.1);
            info!("  click={} {},{} {},{}", self.clicks, (xcell) * SQUARE_SIZE as i32, (ycell) * SQUARE_SIZE as i32, self.x / 4 * 4 - 4, self.y / 4 * 4);
        }
    }

    fn update_current_target_if_reached(&mut self) -> f64 {
        let away = distance(self.x, self.y, self.current_target.0, self.current_target.1);
        if away < SQUARE_SIZE as f64 {
            self.out_of_home = true;
            self.current_target = self.target;
            info!("Updated target {},{}", self.current_target.0, self.current_target.1);
        }
        away
    }

    fn consider_chasing_player_if_directly_visible(&mut self, maze: &mut Vec<Vec<char>>, player_cellx: i32, player_celly: i32, ghost_cellx: i32, ghost_celly: i32) {
        if self.x % 8 != 0 && self.y % 8 != 0 && self.smart_seconds % 30 > 8 {
            let old_x = self.x_dir;
            let old_y = self.y_dir;
            let old_inertia_x = self.inertia_x;
            let old_inertia_y = self.inertia_y;

            let visible = self.check_if_visible_better_route(maze, player_cellx, player_celly, ghost_cellx, ghost_celly);
            if visible {
                if self.check_if_move_ok(maze, self.x + self.x_dir * 3, self.y + self.y_dir * 2) {
                    if self.clicks % 60 == 0 {
                        info!("**** OK TO CHANGE *** {},{} {},{} {}/{}",
                              self.x_dir, self.y_dir, self.inertia_x, self.inertia_y, self.x % 8, self.y % 8);
                    }
                    self.current_target = self.target;
                } else {
                    debug!("**** IGNORE CHANGE *** {},{} {},{}", self.x_dir, self.y_dir, self.inertia_x, self.inertia_y);
                    self.x_dir = old_x;
                    self.y_dir = old_y;
                    self.inertia_x = old_inertia_x;
                    self.inertia_y = old_inertia_y;
                }
            }
        }
    }

    fn set_current_target_to_latest_target(&mut self) {
        if self.smart_seconds % 20 == 19 {
            self.smart_seconds = self.smart_seconds + 1;
            self.current_target = self.target;
            info!("{} Update to player pos * {},{}", self.clicks, self.current_target.0, self.current_target.1);
        }
        if self.clicks % 1800 == rand::thread_rng().gen_range(1, 10) {
            self.smart_seconds = self.smart_seconds + 1;
            let half_square = SQUARE_SIZE as i32 / 2;
            let middle_x = MAZE_WIDTH as i32 / 2;
            let middle_y = MAZE_HEIGHT as i32 / 2;

            self.current_target.0 = rand::thread_rng().gen_range(-1, 1) * half_square + middle_x;
            self.current_target.1 = rand::thread_rng().gen_range(-1, 1) * half_square + middle_y;
            info!("{} Random change {},{}", self.clicks, self.current_target.0, self.current_target.1);
        }
    }

    fn check_if_visible_better_route(&mut self, maze: &mut Vec<Vec<char>>, player_cellx: i32, player_celly: i32, ghost_cellx: i32, ghost_celly: i32) -> bool {
        if Ghost::look_along(maze, ghost_cellx, ghost_celly, player_cellx, player_celly, -1, 0) {
            self.x_dir = -1;
            self.y_dir = 0;
            self.inertia_x = self.x_dir;
            self.inertia_y = 0;
            return true;
        }
        if Ghost::look_along(maze, ghost_cellx, ghost_celly, player_cellx, player_celly, 1, 0) {
            self.x_dir = 1;
            self.y_dir = 0;
            self.inertia_x = self.x_dir;
            self.inertia_y = 0;
            return true;
        }
        if Ghost::look_along(maze, ghost_cellx, ghost_celly, player_cellx, player_celly, 0, -1) {
            self.y_dir = -1;
            self.x_dir = 0;
            self.inertia_x = 0;
            self.inertia_y = self.y_dir;
            return true;
        }

        if Ghost::look_along(maze, ghost_cellx, ghost_celly, player_cellx, player_celly, 0, 1) {
            self.y_dir = 1;
            self.x_dir = 0;
            self.inertia_x = 0;
            self.inertia_y = self.y_dir;
            return true;
        }

        //println!("Not Found is {} {} from {},{}", player_cellx, player_celly,ghost_cellx,ghost_celly);
        return false;
    }

    fn look_along(maze: &mut Vec<Vec<char>>, ghost_cellx: i32, ghost_celly: i32, player_cellx: i32, player_celly: i32, addx: i32, addy: i32) -> bool {
        let mut y = (ghost_celly) as usize;
        let mut x = (ghost_cellx) as usize;
        while x > 0 && x < MAZE_X && y > 0 && y < MAZE_Y && maze[y][x] != 'X' {
            //print!("{},{} ",x,y);
            if player_cellx == x as i32 && player_celly == y as i32 {
                //println!("Found is {} {}", player_cellx, player_celly);
                return true;
            }
            x = (x as i32 + addx) as usize;
            y = (y as i32 + addy) as usize;
        }
        return false;
    }


    fn set_inertia(&mut self, maze: &mut Vec<Vec<char>>) {
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
        if self.inertia_x == 0 && self.inertia_y == 0 && self.x_dir == 0 && self.y_dir == 0 {
            self.change_direction(maze);
        }
    }

    fn apply_inertia(&mut self, maze: &mut Vec<Vec<char>>) {
        if self.inertia_x != 0 {
            let new_x = self.x + self.inertia_x;
            if self.check_if_move_ok(maze, new_x, self.y) {
                self.x = self.x + self.inertia_x;
                self.x_dir = 0;
            } else {
                self.change_direction(maze);
                self.inertia_x = 0;
            }
        }

        if self.inertia_y != 0 {
            let new_y = self.y + self.inertia_y;
            if self.check_if_move_ok(maze, self.x, new_y) {
                self.y = self.y + self.inertia_y;
                self.y_dir = 0;
            } else {
                self.change_direction(maze);
                self.inertia_y = 0;
            }
        }
    }


    fn check_if_move_ok(&mut self, maze: &Vec<Vec<char>>, new_x: i32, new_y: i32) -> bool {
        let half_square = SQUARE_SIZE as i32 / 2;
        let centre_x = new_x + half_square;
        let centre_y = new_y - half_square;

        let xarray = [centre_x - 18 as i32, centre_x, centre_x + 4];
        let yarray = [centre_y - 31, centre_y - 24, centre_y - 19, centre_y - 12, centre_y - 9];
        let mut all_okay = true;
        for y_cell in yarray.iter() {
            for x_cell in xarray.iter() {
                if maze[*y_cell as usize / SQUARE_SIZE][*x_cell as usize / SQUARE_SIZE] == 'X' {
                    all_okay = false;
                }
            }
        }
        return all_okay;
    }
}
