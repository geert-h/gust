use gust_components::ComponentType::{ColliderComponentType, TransformComponentType};
use gust_core::entity::Entity;
use gust_hierarchy::world::World;

pub struct CollisionDetectionSystem;

impl CollisionDetectionSystem {
    pub fn run(delta_time: f32, world: &World) -> Vec<CollisionPair> {
        let mut collision_pairs = Vec::new();
        let collidable_entities: Vec<_> = world.with_components(vec![ColliderComponentType, TransformComponentType]).map(|entity| *entity).collect();

        for (i, entity_a) in collidable_entities.iter().enumerate() {
            for entity_b in collidable_entities.iter().skip(i + 1) {
                if Self::check_collision(world, *entity_a, *entity_b) {
                    collision_pairs.push(CollisionPair { entity_a: *entity_a, entity_b: *entity_b });
                }
            }
        }
        collision_pairs
    }

    fn check_collision(world: &World, entity_a: Entity, entity_b: Entity) -> bool {
        false
    }
}

pub struct CollisionPair {
    entity_a: Entity,
    entity_b: Entity,
}

impl CollisionPair {
    pub fn get_entities(&self) -> (Entity, Entity) {
        (self.entity_a, self.entity_b)
    }
}