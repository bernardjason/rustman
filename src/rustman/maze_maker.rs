use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub const SQUARE_SIZE: usize = 8;
pub const BORDER: usize = 1;
pub const BYTES_PER_PIXEL:usize = 3;
pub const MAZE_WIDTH: usize = 800;
pub const MAZE_HEIGHT: usize = 576;
pub const MAZE_Y: usize = MAZE_HEIGHT / SQUARE_SIZE;
pub const MAZE_X: usize = MAZE_WIDTH / SQUARE_SIZE;

pub fn xy_to_cell(x:i32,y:i32) -> (i32,i32) {
    // *** REMEMBER EVEN THOUGH PLAYER 24 walls etc are on SQUARE BOUNDARY
    let cx = x/SQUARE_SIZE as i32;
    let cy = y/SQUARE_SIZE as i32;

    return(cx,cy);
}

pub fn create_maze_texture<'a>(creator: &'a TextureCreator<WindowContext>, maze:&Vec<Vec<char>>) -> (Texture<'a>,i32 ) {
    let mut count_pills = 0;
    let mut texture: Texture =
        creator.create_texture_streaming(PixelFormatEnum::RGB24, MAZE_WIDTH as u32, (MAZE_HEIGHT + 1) as u32).expect("texture");
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for y in 0..MAZE_Y {
            for x in 0..MAZE_X {
                let c = maze[y][x];
                if c == 'X' {
                    draw_wall_on_texture(buffer, pitch, y, x,Color::BLUE);
                } else if c == 'P' {
                    draw_pill_on_texture(buffer, pitch, y, x,Color::RED);
                } else if c == '*' {
                    draw_pill_on_texture(buffer, pitch, y, x,Color::YELLOW);
                    count_pills = count_pills +1;
                } else if c == '!' {
                    draw_wall_on_texture(buffer, pitch, y, x,Color::RGBA(133, 50, 168, 255));
                }
            }
        }
    }).unwrap();
    return (texture,count_pills);
}
pub fn update_texture<'a>(texture: &mut Texture,x:i32,y:i32) { //} -> &'a Texture<'a> {
    let erase = Rect::new(x,y + BORDER as i32,(SQUARE_SIZE - BORDER) as u32,(SQUARE_SIZE - BORDER) as u32);
    let data:[u8;SQUARE_SIZE*SQUARE_SIZE * BYTES_PER_PIXEL] = [0;SQUARE_SIZE*SQUARE_SIZE*BYTES_PER_PIXEL];
    let pitch:usize = BYTES_PER_PIXEL*SQUARE_SIZE;
    texture.update(erase,&data,pitch).unwrap();
}

fn draw_wall_on_texture(buffer: &mut [u8], pitch: usize, y: usize, x: usize,colour:Color) {
    let xxx = x * SQUARE_SIZE;
    let yyy = y * SQUARE_SIZE;
    for xx in xxx + 0..xxx + SQUARE_SIZE {
        for yy in yyy + 0..yyy + SQUARE_SIZE {
            let offset = yy * pitch + xx * 3;
            buffer[offset] = colour.r;
            buffer[offset + 1] = colour.g;
            buffer[offset + 2] = colour.b;
        }
    }
}

fn draw_pill_on_texture(buffer: &mut [u8], pitch: usize, y: usize, x: usize,colour:Color ) {
    let xxx = x * SQUARE_SIZE;
    let yyy = y * SQUARE_SIZE;
    let startx = BORDER ;
    let endx = SQUARE_SIZE - BORDER ;
    let starty = BORDER ;
    let endy = SQUARE_SIZE - BORDER ;
    for xx in xxx + startx..xxx + endx {
        for yy in yyy + starty..yyy + endy {
            let offset = yy * pitch + xx * 3;
            buffer[offset] = colour.r ;
            buffer[offset + 1] = colour.g;
            buffer[offset + 2] = colour.b;
        }
    }
}


pub fn read_in_maze() -> Vec<Vec<char>> {
    //let mut maze = [['0'; MAZE_X]; MAZE_Y];
    let mut maze = vec![vec![];MAZE_Y+1];
    for y in 0..MAZE_Y {
        maze[y] = vec![' '; MAZE_X];
    }


    if let Ok(lines) = read_lines("artifacts/maze.txt") {
        let mut y = 0;
        for wrapped_line in lines {
            let line = wrapped_line.unwrap();
            let chars = line.chars();
            let mut x = 0;

            for c in chars {
                //maze[y].push(c);
                if c != ' ' {
                    maze[y][x] = c;
                }
                x = x + 1;
            }
            y = y + 1;
        }
    }
    for y in 1..MAZE_Y-2 {
        for x in 1..MAZE_X-1 {
           if maze[y][x] == ' ' &&
               maze[y][x-1] == ' ' &&
               maze[y][x+1] == ' ' &&
               maze[y-1][x] == ' ' &&
               maze[y-1][x-1] == ' ' &&
               maze[y-1][x+1] == ' ' &&
               maze[y+1][x] == ' '  &&
               maze[y+1][x+1] == ' '  &&
               maze[y+1][x-1] == ' '
           {

               maze[y][x] = '*';
           }
        }
    }
    return maze;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
