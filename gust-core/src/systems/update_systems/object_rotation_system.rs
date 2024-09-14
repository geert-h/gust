use gust_hierarchy::world::World;
use gust_math::matrices::mat3::Mat3;

use crate::components::mesh_component::MeshComponent;
use crate::components::player_component::PlayerComponent;
use crate::components::transform_component::TransformComponent;

pub struct ObjectRotationSystem;

impl ObjectRotationSystem {
    pub fn update(world: &mut World, dt: f32) {
        let entities = world.query::<TransformComponent>();


        for entity in entities {
            //retain player entity
            if world.has_component::<PlayerComponent>(entity) ||
                !world.has_component::<MeshComponent>(entity) {
                continue;
            }

            let mut transform = world.get_component_mut::<TransformComponent>(entity).unwrap();

            // rotate the forward vector around the up vector
            let rotation_matrix = Mat3::rotation_matrix(&transform.up, 0.1 * dt);
            transform.forward = rotation_matrix * transform.forward;
        }
    }
}