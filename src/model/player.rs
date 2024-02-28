pub struct Player {
  pub x: i32,
  pub y: i32,
  pub width: u32,
  pub height: u32,
  pub current_texture_index: u32
}

pub fn init_player() -> Player {
  Player {
      x: 0,
      y: 0,
      width: 16,
      height: 16,
      current_texture_index: 0
  }
}