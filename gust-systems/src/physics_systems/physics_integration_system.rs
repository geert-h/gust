use gust_components::Component::{RigidBodyComponent, TransformComponent, VelocityComponent};
use gust_components::ComponentType::{RigidBodyComponentType, TransformComponentType, VelocityComponentType};
use gust_hierarchy::world::World;
use gust_math::vectors::vect3::Vect3;

pub struct PhysicsIntegrationSystem;

impl PhysicsIntegrationSystem {
    pub fn run(delta_time: f32, world: &mut World) {
        let component_types = vec![RigidBodyComponentType, TransformComponentType, VelocityComponentType];
        for (_entity, mut components) in world.query_mut(component_types) {
            let [
            RigidBodyComponent(ref mut rigid_body),
            TransformComponent(ref mut transform),
            VelocityComponent(ref mut velocity)]
                = components[..] else { continue };

            if rigid_body.is_static {
                continue;
            }

            rigid_body.acceleration = rigid_body.forces / rigid_body.mass;
            velocity.velocity += rigid_body.acceleration * delta_time;
            transform.position += velocity.velocity * delta_time;

            rigid_body.forces = Vect3::zeros();
        }
    }
}