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
    
    //Draw grid
    draw_grid(canvas, SCREEN_WIDTH, SCREEN_HEIGHT, None)?;

    //Draw player
    draw_player(game, canvas, texture)?;
    
    //Draw to screen
    canvas.present();

    Ok(())
}

pub fn init_canvas_and_event_queue(
) -> Result<(sdl2::render::Canvas<sdl2::video::Window>, sdl2::EventPump), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("AstroGuy", SCREEN_WIDTH, SCREEN_HEIGHT)
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

    let screen_area = Rect::new(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT);

    canvas.fill_rect(screen_area).unwrap();

    let event_queue = sdl_context.event_pump().unwrap();

    Ok((canvas, event_queue))
}

pub fn draw_player(
    game: &mut Game,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    texture: &sdl2::render::Texture,
) -> Result<(), String> {

    let player_rect = Rect::new(
        game.player.x,
        game.player.y,
        game.player.width,
        game.player.height,
    );
    canvas.copy(texture, None, player_rect).unwrap();
    Ok(())
}

fn draw_grid(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    width: u32,
    height: u32,
    grid_color: Option<Color>,
) -> Result<(), String> {

    canvas.set_draw_color(grid_color.unwrap_or(Color::RGB(255, 255, 255)));

    for x in (15..width-16).step_by(16) {
        canvas.draw_line(
            sdl2::rect::Point::new(x as i32, 0),
            sdl2::rect::Point::new(x as i32, height as i32),
        ).unwrap();
    }
    for y in (15..height-16).step_by(16) {
        canvas.draw_line(
            sdl2::rect::Point::new(0, y as i32),
            sdl2::rect::Point::new(width as i32, y as i32),
        ).unwrap();
    }

    Ok(())
}