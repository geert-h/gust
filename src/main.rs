extern crate sdl2;

mod model;
use model::game::{init_game, SCREEN_HEIGHT, SCREEN_WIDTH};
mod view;
use view::view::draw_game;
mod controller;
use controller::controller::{handle_inputs, update_game};

use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-test", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .maximized()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();

    let screen_area = Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);
    let clear_color = Color::RGB(0, 0, 0);
    canvas.set_draw_color(clear_color);
    canvas
        .set_logical_size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("1kStare.jpg")?;

    let mut game = init_game();

    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();

    canvas.fill_rect(screen_area).unwrap();

    while running {
        running = handle_inputs(&mut game, &mut canvas, &mut event_queue);
        update_game(&mut game);
        draw_game(&mut game, &mut canvas, &texture)?;
    }
    Ok(())
}
