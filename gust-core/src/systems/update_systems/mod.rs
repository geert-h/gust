use gust_hierarchy::world::World;

use crate::components::player_component::PlayerComponent;
use crate::handlers::input_handler::InputHandler;

mod player_update_system;

pub struct UpdateSystem {
    pub delta_time: f32,
    pub game_input: InputHandler,
    pub world: World,
}

impl UpdateSystem {
    pub fn new(delta_time: f32, game_input: InputHandler, world: World) -> Self {
        UpdateSystem {
            delta_time,
            game_input,
            world,
        }
    }

    pub fn update(&mut self) {
        // First update the player
        let player = self.world.query_one::<PlayerComponent>();

        if let Some(player) = player {
            // Update the player's transform
            player_update_system::PlayerUpdateSystem::update(player, &self.delta_time, &mut self.world, &self.game_input);
        }
    }
}
