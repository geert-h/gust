extern crate sdl2;

mod model;
use model::game::init_game;
mod view;
use view::view::init_canvas_and_event_queue;
mod controller;
use controller::controller::game_loop;

fn main() -> Result<(), String> {
    let (mut canvas, mut event_queue) = init_canvas_and_event_queue()?;

    let mut game = init_game();

    game_loop(&mut game, &mut canvas, &mut event_queue)?;

    Ok(())
}