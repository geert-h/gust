use crate::{
    model::game::{self, SCREEN_HEIGHT, SCREEN_WIDTH},
    view::view::{draw_game, init_canvas_and_event_queue},
};

use game::{Game, Player, Runner};
use sdl2::{image::LoadTexture, rect::Rect};

use super::controller::{handle_inputs, update_game};

impl Runner for Game {
    fn run(&mut self) {
        let (mut canvas, mut event_queue) = init_canvas_and_event_queue().unwrap();

        let screen_area = Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);

        let texture_creator = canvas.texture_creator();
        let texture = texture_creator.load_texture("assets/astro.png").unwrap();

        let player = Player::new(0, 0, 16, 16);

        let mut game = Game::new(player, SCREEN_WIDTH, SCREEN_HEIGHT);

        let mut running = true;

        canvas.fill_rect(screen_area).unwrap();

        while running {
            running = handle_inputs(&mut game, &mut canvas, &mut event_queue);
            update_game(&mut game);
            draw_game(&mut game, &mut canvas, &texture).unwrap();
        }
    }
}
