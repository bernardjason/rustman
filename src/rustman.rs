extern crate pretty_env_logger;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::gfx::framerate::FPSManager;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureCreator;

use crate::rustman::ghost_maker::Ghost;
use crate::rustman::maze_maker::{MAZE_HEIGHT, MAZE_WIDTH, SQUARE_SIZE};
use crate::rustman::player_maker::Player;
use crate::rustman::sound::{DEAD, GOTGHOST, load_sound};

use self::sdl2::pixels::PixelFormatEnum;
use self::sdl2::render::{Canvas, Texture};
use self::sdl2::ttf::Font;
use self::sdl2::video::{Window, WindowContext};
use self::sdl2::surface::Surface;

mod player_maker;
mod maze_maker;
mod ghost_maker;
mod sound;

pub const YELLOW: Color = Color::RGBA(255, 255, 0, 255);

enum GAME_STATE {
    START,
    PLAY,
    END,
}

pub fn rustman() -> Result<(), String> {
    pretty_env_logger::init();
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut frame_control = FPSManager::new();

    load_sound(&sdl_context);

    let mut space_pressed = false;

    frame_control.set_framerate(60)?;
    let mut player = player_maker::Player::new();

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path = "artifacts/OpenSans-Light.ttf";
    let mut font = ttf_context.load_font(font_path, 32)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);


    let mut ghosts = vec![
        ghost_maker::Ghost::new(0, -1,
                                MAZE_WIDTH as i32 / 2 - 20,
                                MAZE_HEIGHT as i32 / 2, 0, MAZE_HEIGHT as i32),
        ghost_maker::Ghost::new(0, -1,
                                MAZE_WIDTH as i32 / 2 - 60,
                                MAZE_HEIGHT as i32 / 2, MAZE_WIDTH as i32, MAZE_HEIGHT as i32),
        ghost_maker::Ghost::new(0, -1,
                                MAZE_WIDTH as i32 / 2 + 30,
                                MAZE_HEIGHT as i32 / 2, MAZE_WIDTH as i32, MAZE_HEIGHT as i32 / 3),
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

    let TITLE_HEIGHT = 150;

    let mut get_pixels_surface = font.render("Berniesoft").blended(Color::WHITE).map_err(|e| e.to_string())?;
    let bernie_soft_title = make_title_texture(TITLE_HEIGHT, Color::YELLOW, &texture_creator, get_pixels_surface);

    get_pixels_surface = font.render("Rustman").blended(Color::WHITE).map_err(|e| e.to_string())?;
    let rustman_title = make_title_texture(TITLE_HEIGHT, Color::CYAN, &texture_creator, get_pixels_surface);

    get_pixels_surface = font.render("again?").blended(Color::WHITE).map_err(|e| e.to_string())?;
    let play_again_title = make_title_texture(TITLE_HEIGHT, Color::CYAN, &texture_creator, get_pixels_surface);

    //let mut maze_data: Vec<Vec<char>> , count_pills:i32 = maze_maker::read_in_maze();
    let mut maze_data: Vec<Vec<char>>  = maze_maker::read_in_maze();
    let mut maze_stuff = maze_maker::create_maze_texture(&texture_creator, &maze_data);

    let mut count_pills = maze_stuff.1;
    let mut maze = maze_stuff.0;


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
    let mut game_state = GAME_STATE::START;

    let mut event_pump = sdl_context.event_pump()?;
    'gui_loop: loop {
        sound::pause_any_finished_sounds();
        clicks = clicks + 1;

        space_pressed = false;

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
                Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {
                    space_pressed = true;
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
        match game_state {
            GAME_STATE::START => {
                draw_start_title(&mut canvas, TITLE_HEIGHT, &bernie_soft_title, &rustman_title, &mut clicks);
                if clicks > 300 {
                    game_state = GAME_STATE::PLAY;
                }
            }
            GAME_STATE::PLAY => {
                let (mut ghost_which_texture, ghost_flip_counter) = player_logic(&mut player, &mut ghosts, &mut maze_data, ghost_which_texture, ghost_flip_counter, &mut clicks);

                let (eaten, wherex, wherey) = player.render(&mut maze_data);

                if eaten {
                    maze_maker::update_texture(&mut maze, wherex, wherey);
                    count_pills=count_pills-1;
                }

                live_and_dir_logic(&mut player, &mut ghosts);

                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.clear();

                draw_score(count_pills,&mut player, &mut font, &mut canvas, &texture_creator);

                if count_pills <= 0 {
                    player.player_reset();
                    for i in 0..ghosts.len() {
                        ghosts[i].ghost_reset();
                    }
                    maze_data = maze_maker::read_in_maze();
                    let maze_stuff = maze_maker::create_maze_texture(&texture_creator, &maze_data);
                    count_pills = maze_stuff.1;
                    maze = maze_stuff.0;
                }
                if player.lives <= 0 {
                    game_state = GAME_STATE::END;
                }
                ghosts = draw_game_screen(&mut player, ghosts, &mut canvas, &mut maze, &player_texture, &mut ghost_which_texture, &ghost_textures)
            }
            GAME_STATE::END => {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.clear();

                draw_score(count_pills,&mut player, &mut font, &mut canvas, &texture_creator);
                canvas.copy(&play_again_title, None, Some(Rect::new(0, MAZE_HEIGHT as i32/2, 800, TITLE_HEIGHT))).unwrap();

                if space_pressed {
                    player.lives=3;
                    player.score=0;
                    game_state = GAME_STATE::PLAY;
                    player.player_reset();
                    for i in 0..ghosts.len() {
                        ghosts[i].ghost_reset();
                    }
                    maze_data = maze_maker::read_in_maze();
                    let maze_stuff = maze_maker::create_maze_texture(&texture_creator, &maze_data);
                    count_pills = maze_stuff.1;
                    maze = maze_stuff.0;
                }
            }
        }


        canvas.present();

        frame_control.delay();
    }

    Ok(())
}

fn draw_game_screen(mut player: &mut Player, mut ghosts: Vec<Ghost>, canvas: &mut Canvas<Window>, maze: &mut Texture, player_texture: &[Texture; 5], ghost_which_texture: &mut usize, ghost_textures: &[Texture; 2]) -> Vec<Ghost> {
    canvas.copy(&maze, None, Some(Rect::new(0, 24, 800, 576))).unwrap();

    for i in 0..ghosts.len() {
        canvas.copy(&ghost_textures[*ghost_which_texture], None, Some(
            Rect::new(ghosts[i].x - ghost_maker::WIDTH as i32 / 2, ghosts[i].y - ghost_maker::HEIGHT as i32 / 2, ghost_maker::WIDTH, ghost_maker::HEIGHT)
        )).unwrap();
    }

    canvas.copy(&player_texture[player.direction], None, Some(
        Rect::new(player.x - player_maker::WIDTH as i32 / 2, player.y - player_maker::HEIGHT as i32 / 2, player_maker::WIDTH, player_maker::HEIGHT)
    )).unwrap();

    return ghosts;
}

fn draw_start_title(canvas: &mut Canvas<Window>, TITLE_HEIGHT: u32, bernie_soft_title: &Texture, rustman_title: &Texture, clicks: &mut i32) {
    let mut title_y = 100;
    let pause = 60;

    if (*clicks > pause) {
        if *clicks < 300 {
            title_y = title_y - *clicks + pause;
        } else {
            title_y = -300;
        }
    }

    canvas.copy(&bernie_soft_title, None, Some(Rect::new(0, title_y, 800, TITLE_HEIGHT))).unwrap();
    canvas.copy(&rustman_title, None, Some(Rect::new(0, MAZE_HEIGHT as i32 / 2 - TITLE_HEIGHT as i32 / 2, 800, TITLE_HEIGHT))).unwrap();
}

fn make_title_texture<'a>(height: u32, colour: Color, texture_creator: &'a TextureCreator<WindowContext>, mut get_pixels_surface: Surface) -> Texture<'a> {
    let size = get_pixels_surface.width() * get_pixels_surface.height();
    let mut pixel_buffer = Vec::with_capacity(size as usize);
    pixel_buffer.resize(size as usize, 0);

    let OFF_TOP = 11;
    get_pixels_surface.with_lock(|buffer: &[u8]| {
        for y in OFF_TOP..get_pixels_surface.height() {
            for x in 0..get_pixels_surface.width() {
                let index = (y * get_pixels_surface.pitch() + x * 4) as usize;
                let val = buffer[index + 3];
                if val > 0 {
                    let index = ((y - OFF_TOP) * get_pixels_surface.width() + x) as usize;
                    pixel_buffer[index] = 1;
                }
            }
        }
    }
    );
    let mut title_by: Texture =
        texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, MAZE_WIDTH as u32, height).expect("texture");
    title_by.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        let GAP = 4;
        let mut slant = 0;

        //for i in 0..( pitch * (50) as usize * 3)  { buffer[i]=255; }

        for oppy in 1..get_pixels_surface.height() {
            let y = get_pixels_surface.height() - oppy - 1;
            for x in 0..get_pixels_surface.width() {
                let index = y * get_pixels_surface.width() + x;
                let val = pixel_buffer[index as usize];
                if val == 1 {
                    let size = GAP / 2;
                    for yy in y * GAP..y * GAP + size {
                        for xx in x * GAP..x * GAP + size {
                            let offset = ((yy) * pitch as u32 + (xx + slant) * 3) as usize;
                            buffer[offset] = colour.r;
                            buffer[offset + 1] = colour.g;
                            buffer[offset + 2] = colour.b;
                        }
                    }
                }
            }
            slant = slant + 2;
        }
    });
    title_by
}

fn draw_score(count_pills:i32,player: &mut Player, font: &mut Font, canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>) {
    let score_text = format!("Lives {lives} Score {score} pills left {pills}",
                             lives = player.lives, score = player.score,pills =count_pills);
    let font_surface = font.render(score_text.as_str()).blended(YELLOW).unwrap();
    //.map_err(|e| e.to_string())?;
    let texture_text = font_surface.as_texture(&texture_creator).unwrap();
    //.map_err(|e| e.to_string())?;
    canvas.copy(&texture_text, None, Rect::new(0, -4, 150, 32));
}

fn live_and_dir_logic(player: &mut Player, ghosts: &mut Vec<Ghost>) {
    let player_cellx = player.x / SQUARE_SIZE as i32;
    let player_celly = player.y / SQUARE_SIZE as i32;
    for i in 0..ghosts.len() {
        // print!("{},{} ",ghosts[i].x / SQUARE_SIZE as i32,ghosts[i].y/SQUARE_SIZE as i32)     ;
        let cellx = ghosts[i].x / SQUARE_SIZE as i32;
        let celly = ghosts[i].y / SQUARE_SIZE as i32;
        if cellx == player_cellx && celly == player_celly {
            if player.super_pill == 0 {
                player.player_reset();
                player.lives=player.lives-1;
                ghosts[i].ghost_reset();
                sound::play(DEAD);
            } else {
                ghosts[i].ghost_reset();
                sound::play(GOTGHOST);
            }
        }
    }
}

fn player_logic(player: &mut Player, ghosts: &mut Vec<Ghost>, mut maze_data: &mut Vec<Vec<char>>, ghost_which_texture: usize, ghost_flip_counter: usize, clicks: &mut i32) -> (usize, usize) {
    let mut ghost_which_texture: usize = ghost_which_texture;
    let mut ghost_flip_counter: usize = ghost_flip_counter;

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
                if *clicks % 60 == 0 {
                    ghost_flip_counter = ghost_flip_counter + 1;
                }
                ghost_which_texture = (ghost_flip_counter % 2) as usize;
            } else {
                ghost_which_texture = 1;
            }
        }
    }
    return (ghost_which_texture, ghost_flip_counter);
}