use std::cmp;

use crate::model;
use crate::view::view::draw_game;

use model::game::{Game, SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;

use sdl2::video::Window;
use sdl2::render::Canvas;

use sdl2::image::LoadTexture;

pub fn handle_inputs(
    game: &mut Game,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    event_queue: &mut sdl2::EventPump,
) -> bool {
    let mut running = true;

    for event in event_queue.poll_iter() {
        running = handle_input(game, canvas, event);
    }

    running
}

fn handle_input(
    game: &mut Game,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    event: Event,
) -> bool {
    let mut running = true;

    match event {
        Event::Quit { .. } => {
            running = false;
        }
        Event::MouseButtonDown {
            mouse_btn, x, y, ..
        } => {
            if mouse_btn == MouseButton::Left {
                game.player.x = x - (game.player.width / 2) as i32;
                game.player.y = y - (game.player.height / 2) as i32;
            }
        }

        Event::MouseWheel { y, .. } => {
            if y < 1 {
                game.player.width = cmp::max(5, game.player.width - 5);
                game.player.height = game.player.width;
            } else {
                game.player.width =
                    cmp::min(game.player.width + 5, cmp::min(SCREEN_WIDTH, SCREEN_HEIGHT));
                game.player.height = game.player.width;
            }

            game.player.x = game.player.x;
            game.player.y = game.player.y;
        }

        Event::MouseMotion {
            x, y, mousestate, ..
        } => {
            game.player.x = x - (game.player.width / 2) as i32;
            game.player.y = y - (game.player.height / 2) as i32;
        }

        Event::KeyDown {
            keycode: Some(keycode),
            ..
        } => {
            game.pressed_keys.insert(keycode);
        }

        Event::KeyUp {
            keycode: Some(keycode),
            ..
        } => {
            game.pressed_keys.remove(&keycode);
        }

        Event::Window {
            win_event: WindowEvent::Resized(w, h),
            ..
        } => {
            canvas.window_mut().set_size(w as u32, h as u32).unwrap();
        }

        _ => {}
    }

    running
}

pub fn update_game(game: &mut Game) {
    for keycode in &game.pressed_keys {
        match keycode {
            Keycode::W => {
                game.player.y -= 1;
            }
            Keycode::S => {
                game.player.y += 1;
            }
            Keycode::A => {
                game.player.x -= 1;
            }
            Keycode::D => {
                game.player.x += 1;
            }
            _ => {}
        }
    }

    //bound player to screen
    game.player.x = cmp::max(game.player.x, 0);
    game.player.x = cmp::min(
        game.player.x,
        (SCREEN_WIDTH - game.player.width as u32) as i32,
    );

    game.player.y = cmp::max(game.player.y, 0);
    game.player.y = cmp::min(
        game.player.y,
        (SCREEN_HEIGHT - game.player.height as u32) as i32,
    );
}

pub fn game_loop(mut game : &mut Game, mut canvas : &mut  Canvas<Window>, mut event_queue: &mut sdl2::EventPump) -> Result<(), String> {
    let mut running = true;

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/astro.png")?;

    while running {
        running = handle_inputs(&mut game, &mut canvas, &mut event_queue);
        update_game(&mut game);
        draw_game(&mut game, &mut canvas, &texture)?;
    }

    Ok(())
}
