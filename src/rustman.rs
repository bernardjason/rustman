extern crate sdl2;
extern crate pretty_env_logger;

mod player_maker;
mod maze_maker;
mod ghost_maker;
mod sound;

use sdl2::event::Event;
use sdl2::gfx::framerate::FPSManager;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use crate::rustman::maze_maker::{MAZE_WIDTH, MAZE_HEIGHT, SQUARE_SIZE};
use crate::rustman::sound::{load_sound, DEAD, GOTGHOST};

pub const YELLOW: Color = Color::RGBA(255, 255, 0, 255);

pub fn rustman() -> Result<(), String> {
    pretty_env_logger::init();
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut frame_control = FPSManager::new();

    load_sound(&sdl_context);


    frame_control.set_framerate(60)?;
    let mut player = player_maker::Player::new();

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path = "artifacts/OpenSans-Light.ttf";
    let mut font = ttf_context.load_font(font_path, 256)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let mut ghosts = vec![

        ghost_maker::Ghost::new(0, -1,
                                MAZE_WIDTH  as i32/2 - 20,
                                MAZE_HEIGHT as i32 /2 ,0,MAZE_HEIGHT as i32),
        ghost_maker::Ghost::new(0, -1,
                                MAZE_WIDTH  as i32/2 - 60,
                                MAZE_HEIGHT as i32 /2 ,MAZE_WIDTH  as i32 , MAZE_HEIGHT as i32 ),
        ghost_maker::Ghost::new(0, -1,
                                MAZE_WIDTH  as i32/2 + 30,
                                MAZE_HEIGHT as i32 /2 ,MAZE_WIDTH  as i32  , MAZE_HEIGHT as i32/3),

    ];

    debug!("PLAYER {},{}", player.x, player.y);
    debug!("GHOST 0 {},{}", ghosts[0].x, ghosts[0].y);

    debug!("Lives {}", player.lives);
    let window = video_subsystem.window("rustman", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build()
        .map_err(|e| e.to_string())?;

    let texture_creator: TextureCreator<_> = canvas.texture_creator();


    let mut maze_data: Vec<Vec<char>> = maze_maker::read_in_maze();
    //let (mut maze_data,mut maze) = maze_maker::create_maze_texture(&texture_creator);
    let mut maze = maze_maker::create_maze_texture(&texture_creator, &maze_data);

    //let player_texutre = player_maker::create_player_texture(&texture_creator);

    let player_texture = [
        texture_creator.load_texture("artifacts/scamanleft.png")?,
        texture_creator.load_texture("artifacts/scamanright.png")?,
        texture_creator.load_texture("artifacts/scamanup.png")?,
        texture_creator.load_texture("artifacts/scamandown.png")?,
        texture_creator.load_texture("artifacts/scamaneat.png")?,
    ];
    let mut ghost_which_texture: usize = 0;
    let mut ghost_flip_counter: usize = 0;
    let ghost_textures = [
        texture_creator.load_texture("artifacts/ghostman.png")?,
        texture_creator.load_texture("artifacts/ghostmaneatme.png")?,
    ];

    let mut clicks: i32 = 0;

    let mut event_pump = sdl_context.event_pump()?;
    'gui_loop: loop {
        sound::pause_any_finished_sounds();
        clicks = clicks + 1;
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'gui_loop;
                }
                Event::KeyUp { .. } => {
                    //player.x_dir = 0;
                    //player.y_dir = 0;
                }
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {
                    player.x_dir = -1;
                }
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {
                    player.x_dir = 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {
                    player.y_dir = -1;
                }
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {
                    player.y_dir = 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Num1), repeat: false, .. } => {
                    for i in 0..ghosts.len() {
                        ghosts[i].stop = true;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Num2), repeat: false, .. } => {
                    for i in 0..ghosts.len() {
                        ghosts[i].stop = false;
                    }
                }
                _ => {}
            }
        }

        for i in 0..ghosts.len() {
            if player.super_pill <= 0 {
                ghost_which_texture = 0;
                ghosts[i].target.0 = player.x;
                ghosts[i].target.1 = player.y;
            } else {
                ghosts[i].target.0 = MAZE_WIDTH as i32 - player.x;
                ghosts[i].target.1 = MAZE_HEIGHT as i32 - player.y;
            }
            ghosts[i].render(&mut maze_data);
            if player.super_pill > 0 {
                if player.super_pill < (player_maker::SUPER_PILL_TIME as f64 * 0.1) as i32 {
                    if clicks % 60 == 0 {
                        ghost_flip_counter = ghost_flip_counter + 1;
                    }
                    ghost_which_texture = (ghost_flip_counter % 2) as usize;
                } else {
                    ghost_which_texture = 1;
                }
            }
        }

        let (eaten, wherex, wherey) = player.render(&mut maze_data);
        if eaten {
            maze_maker::update_texture(&mut maze, wherex, wherey);
        }


        let player_cellx = player.x / SQUARE_SIZE as i32;
        let player_celly = player.y / SQUARE_SIZE as i32;
        for i in 0..ghosts.len() {
            // print!("{},{} ",ghosts[i].x / SQUARE_SIZE as i32,ghosts[i].y/SQUARE_SIZE as i32)     ;
            let cellx = ghosts[i].x / SQUARE_SIZE as i32;
            let celly = ghosts[i].y / SQUARE_SIZE as i32;
            if cellx == player_cellx && celly == player_celly {
                if player.super_pill == 0 {
                    player.player_reset();
                    ghosts[i].ghost_reset();
                    sound::play(DEAD);
                } else {
                    ghosts[i].ghost_reset();
                    sound::play(GOTGHOST);
                }
            }
        }



        let score_text = format!("Lives {lives} Score {score} {x},{y}",
                                 lives = player.lives, score = player.score, x = player.x, y = player.y);
        let font_surface = font.render(score_text.as_str()).blended(YELLOW).map_err(|e| e.to_string())?;
        let texture_text = font_surface.as_texture(&texture_creator).map_err(|e| e.to_string())?;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.copy(&maze, None, Some(Rect::new(0, 24, 800, 576))).unwrap();

        for i in 0..ghosts.len() {
            canvas.copy(&ghost_textures[ghost_which_texture], None, Some(
                Rect::new(ghosts[i].x - ghost_maker::WIDTH as i32 / 2, ghosts[i].y - ghost_maker::HEIGHT as i32 / 2, ghost_maker::WIDTH, ghost_maker::HEIGHT)
            )).unwrap();
        }

        canvas.copy(&player_texture[player.direction], None, Some(
            Rect::new(player.x - player_maker::WIDTH as i32 / 2, player.y - player_maker::HEIGHT as i32 / 2, player_maker::WIDTH, player_maker::HEIGHT)
        )).unwrap();

        canvas.copy(&texture_text, None, Rect::new(0, -4, 150, 32))?;

        canvas.present();

        frame_control.delay();
    }

    Ok(())
}