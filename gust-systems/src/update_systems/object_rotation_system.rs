use gust_components::Component::TransformComponent;
use gust_components::ComponentType::TransformComponentType;
use gust_hierarchy::world::World;
use gust_math::matrices::mat3::Mat3;

pub struct ObjectRotationSystem;

impl ObjectRotationSystem {
    pub fn update(world: &mut World, dt: f32) {
        for (_entity, component) in world.query_one_mut(TransformComponentType) {
            let TransformComponent(object_transform) = component else { return; };
            let rotation_matrix = Mat3::rotation_matrix(&object_transform.forward, 0.1 * dt);
            object_transform.up = rotation_matrix * object_transform.up;
        }
    }
}