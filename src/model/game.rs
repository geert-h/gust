use sdl2::keyboard::Keycode;
use std::collections::HashSet;

pub static SCREEN_WIDTH: u32 = 256;
pub static SCREEN_HEIGHT: u32 = 144;

pub struct Game {
    pub player: Player,
    pub boxes: Vec<Box>,
    pub pressed_keys: HashSet<Keycode>,
    pub width: u32,
    pub height: u32,
}

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub struct Box {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub fn init_player() -> Player {
    Player {
        x: 0,
        y: 0,
        width: 16,
        height: 16,
    }
}

pub fn init_boxes() -> Vec<Box> {
    Vec::new()
}

pub fn init_game() -> Game {
    Game {
        player: init_player(),
        boxes: init_boxes(),
        pressed_keys: HashSet::new(),
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
    }
}
