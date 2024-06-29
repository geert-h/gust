use winit::keyboard::{Key};

struct InputHandler {
    keyboard_input : KeyBoardInput,
    mouse_input : MouseInput,
}

struct KeyBoardInput {
    pressed_keys: Vec<PressedKey>
}

struct MouseInput {
    mouse_position: (f32, f32),
    mouse_delta: (f32, f32),
    lmb_pressed: bool,
    rmb_pressed: bool,
}

struct PressedKey {
    key : Key,
    pressed_duration : f32,
}

// I want a function that for each tick of the game loop handles the input
// and updates the state of the game accordingly
impl InputHandler {
    fn handle_input(&mut self) {
        // handle keyboard input
        for pressed_key in self.keyboard_input.pressed_keys.iter_mut() {
            pressed_key.pressed_duration += 1.0;
        }

        // handle mouse input
        self.mouse_input.mouse_position.0 += self.mouse_input.mouse_delta.0;
        self.mouse_input.mouse_position.1 += self.mouse_input.mouse_delta.1;
    }
}