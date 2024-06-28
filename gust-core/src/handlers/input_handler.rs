use winit::keyboard::{Key};

struct InputHandler {

}

struct KeyBoardInput {
    pressed_keys: Vec<PressedKey>
}

struct MouseInput {
    mouse_position: (f32, f32),
    mouse_delta: (f32, f32),
}

struct PressedKey {
    key : Key,
    pressed_duration : f32,
}