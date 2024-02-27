use crate::model::game::{Game, SCREEN_HEIGHT, SCREEN_WIDTH};
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub fn draw_game(
    game: &mut Game,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    texture: &sdl2::render::Texture,
) -> Result<(), String> {
    let clear_color = Color::RGB(0, 0, 0);

    //Draw all boxes
    canvas.set_draw_color(clear_color);
    canvas.clear();
    for rect in &game.boxes {
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        let rect = Rect::new(rect.x, rect.y, rect.width, rect.height);
        canvas.fill_rect(rect).unwrap();
    }

    //Draw player
    //Bind texture to player
    let player_rect = Rect::new(
        game.player.x,
        game.player.y,
        game.player.width,
        game.player.height,
    );
    canvas.copy(texture, None, player_rect).unwrap();

    //Draw to screen
    canvas.present();

    Ok(())
}

pub fn init_canvas_and_event_queue(
) -> Result<(sdl2::render::Canvas<sdl2::video::Window>, sdl2::EventPump), String> {
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

    let clear_color = Color::RGB(0, 0, 0);
    canvas.set_draw_color(clear_color);
    canvas
        .set_logical_size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .unwrap();

    let event_queue = sdl_context.event_pump().unwrap();

    Ok((canvas, event_queue))
}
