extern crate sdl2;

mod model;
use model::game::{init_game, SCREEN_HEIGHT, SCREEN_WIDTH};
mod view;
use view::view::{draw_game, init_canvas_and_event_queue};
mod controller;
use controller::controller::{handle_inputs, update_game};

use sdl2::image::LoadTexture;
use sdl2::rect::Rect;

fn main() -> Result<(), String> {
    let (mut canvas, mut event_queue) = init_canvas_and_event_queue()?;

    let screen_area = Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("astro.png")?;

    let mut game = init_game();

    let mut running = true;

    canvas.fill_rect(screen_area).unwrap();

    while running {
        running = handle_inputs(&mut game, &mut canvas, &mut event_queue);
        update_game(&mut game);
        draw_game(&mut game, &mut canvas, &texture)?;
    }
    Ok(())
}
