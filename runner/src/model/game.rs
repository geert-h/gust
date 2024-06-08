use sdl2::keyboard::Keycode;
use std::collections::HashSet;

pub static SCREEN_WIDTH: u32 = 256;
pub static SCREEN_HEIGHT: u32 = 144;

pub struct Game {
    pub player: Player,
    pub pressed_keys: HashSet<Keycode>,
    pub width: u32,
    pub height: u32,
}

impl Game {
    pub fn new(player: Player, width: u32, height: u32) -> Game {
        Game {
            player,
            pressed_keys: HashSet::new(),
            width,
            height,
        }
    }
}

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Player {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Player {
        Player {
            x,
            y,
            width,
            height,
        }
    }
}

pub trait Runner {
    fn run(&mut self);
}
