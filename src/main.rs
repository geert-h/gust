extern crate sdl2;

use std::collections::HashSet;
use std::cmp;

use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;

static SCREEN_WIDTH: u32 = 800;
static SCREEN_HEIGHT: u32 = 600;

struct Game {
    delta_time: f32,
    player: Player,
    boxes: Vec<Box>,
    pressed_keys: HashSet<Keycode>
}

struct Player {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

struct Box {
    x: i32,
    y: i32,
    width: u32,
    height: u32,

}

fn main() -> Result<(), String> {

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("rust-test", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();
    
    let screen_area = Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
    let clear_color = Color::RGB(0, 0, 0);
    canvas.set_draw_color(clear_color);

    let mut game = init_game();

    let mut rect_size: u32 = 30;

    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();

    canvas.fill_rect(screen_area).unwrap();

    while running {
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit {..} => {
                    running = false;
                },
                Event::MouseButtonDown { mouse_btn, x, y, .. } => {
                    if mouse_btn == MouseButton::Left {
                        let new_rect = Box {
                            x: x - (rect_size / 2) as i32,
                            y: y - (rect_size / 2) as i32,
                            width: rect_size,
                            height: rect_size,
                        };
                        game.boxes.push(new_rect);
                    }
                }

                Event::MouseWheel {y, ..} => {
                    if y < 1 {
                        if rect_size > 5 {
                            rect_size -= 5;
                        }
                    } else {
                        rect_size += 5;
                    }
                    
                    game.player.width = rect_size;
                    game.player.height = rect_size;

                    game.player.x = game.player.x;
                    game.player.y = game.player.y;
                }
                Event::MouseMotion { x, y, mousestate, .. } => {
                    if mousestate.left() {
                        let new_rect = Box {
                            x: x - (rect_size / 2) as i32,
                            y: y - (rect_size / 2) as i32,
                            width: rect_size,
                            height: rect_size,
                        };
                        game.boxes.push(new_rect);
                    }
                    game.player.x = x - (game.player.width / 2) as i32;
                    game.player.y = y - (game.player.height / 2) as i32;
                },
                Event::KeyDown { keycode: Some(keycode), .. } => {
                    game.pressed_keys.insert(keycode);
                },
                Event::KeyUp { keycode: Some(keycode), .. } => {
                    game.pressed_keys.remove(&keycode);
                },
                Event::User {timestamp, ..} => {
                    game.delta_time = timestamp as f32;
                },
                _ => {}
            }
        }
        update_game(&mut game);
        draw_game(&mut game, &mut canvas)?;
    }

    Ok(())
}

fn update_game(game: &mut Game){
    for keycode in &game.pressed_keys {
        match keycode {
            Keycode::W => {
                game.player.y -= 1;
            },
            Keycode::S => {
                game.player.y += 1;
            },
            Keycode::A => {
                game.player.x -= 1;
            },
            Keycode::D => {
                game.player.x += 1;
            },
            _ => {}
        }
    }
    
    //bound player to screen
    game.player.x = cmp::max(game.player.x, 0);
    game.player.x = cmp::min(game.player.x, (SCREEN_WIDTH - game.player.width as u32) as i32);

    game.player.y = cmp::max(game.player.y, 0);
    game.player.y = cmp::min(game.player.y, (SCREEN_HEIGHT - game.player.height as u32) as i32);
}

fn draw_game(game: &mut Game, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> { 
    let clear_color = Color::RGB(0, 0, 0);

    canvas.set_draw_color(clear_color);
    canvas.clear();
    for rect in &game.boxes {
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        let rect = Rect::new(rect.x, rect.y, rect.width, rect.height);
        canvas.fill_rect(rect).unwrap();
    }
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    let rect = Rect::new(game.player.x, game.player.y, game.player.width, game.player.height);
    canvas.fill_rect(rect).unwrap();
    canvas.present();

    Ok(())
}

fn init_player() -> Player {
    Player {
        x: 0,
        y: 0,
        width: 30,
        height: 30,
    }
}

fn init_boxes() -> Vec<Box> {
    Vec::new()
}

fn init_game() -> Game {
    Game {
        player: init_player(),
        boxes: init_boxes(),
        pressed_keys: HashSet::new(),
        delta_time: 0.0,
    }
}