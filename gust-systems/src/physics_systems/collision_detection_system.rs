use gust_components::Component::{ColliderComponent, TransformComponent};
use gust_components::ComponentType::{ColliderComponentType, TransformComponentType};
use gust_components::physics::collider_component::ColliderType;
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
        // Get components for both entities
        let [ColliderComponent(ref collider_a), TransformComponent(ref transform_a)] =
            world.get_components(entity_a, vec![ColliderComponentType, TransformComponentType]).unwrap()[..]
        else { return false };

        let [ColliderComponent(ref collider_b), TransformComponent(ref transform_b)] =
            world.get_components(entity_b, vec![ColliderComponentType, TransformComponentType]).unwrap()[..]
        else { return false };

        // Currently only supports AABB vs. AABB collision
        match (&collider_a.collider_type, &collider_b.collider_type) {
            (ColliderType::AABB { min: min_a, max: max_a }, ColliderType::AABB { min: min_b, max: max_b }) => {
                // Transform the collider bounds to world space
                let a_min_world = transform_a.position + *min_a;
                let a_max_world = transform_a.position + *max_a;

                let b_min_world = transform_b.position + *min_b;
                let b_max_world = transform_b.position + *max_b;

                // Check for overlap
                (a_min_world.x <= b_max_world.x && a_max_world.x >= b_min_world.x) &&
                    (a_min_world.y <= b_max_world.y && a_max_world.y >= b_min_world.y) &&
                    (a_min_world.z <= b_max_world.z && a_max_world.z >= b_min_world.z)
            }
            // Add more collision type checks here (e.g., Sphere vs. Sphere)
            _ => false, // For now, return false for unsupported collider types
        }
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