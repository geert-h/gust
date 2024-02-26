use crate::model::game::Game;
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
