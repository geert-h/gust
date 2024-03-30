extern crate sdl2;

mod model;
use model::game::{Game, Player, SCREEN_HEIGHT, SCREEN_WIDTH};
mod controller;
mod view;

use model::game::Runner;

fn main() -> Result<(), String> {
    let mut game = Game::new(Player::new(0, 0, 16, 16), SCREEN_WIDTH, SCREEN_HEIGHT);
    game.run();
    Ok(())
}
