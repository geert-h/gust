use gust_components::Component::{RigidBodyComponent, TransformComponent, VelocityComponent};
use gust_components::ComponentType::{RigidBodyComponentType, TransformComponentType, VelocityComponentType};
use gust_hierarchy::world::World;

use crate::physics_systems::collision_detection_system::CollisionPair;

pub struct CollisionResolutionSystem;

impl CollisionResolutionSystem {
    pub fn run(delta_time: f32, world: &mut World, collision_pairs: Vec<CollisionPair>) {
        let component_types = vec![TransformComponentType, RigidBodyComponentType, VelocityComponentType];
        for collision in collision_pairs {
            let (entity_a, entity_b) = collision.get_entities();

            let [TransformComponent(ref mut transform_a),
            RigidBodyComponent(ref mut rigid_body_a),
            VelocityComponent(ref mut velocity_a)]
                = world.get_components_mut(entity_a, component_types.clone())
                .unwrap()[..] else { continue };

            let is_static_a = rigid_body_a.is_static;

            let [TransformComponent(ref mut transform_b),
            RigidBodyComponent(ref mut rigid_body_b),
            VelocityComponent(ref mut velocity_b)]
                = world.get_components_mut(entity_b, component_types.clone())
                .unwrap()[..] else { continue };

            if is_static_a && rigid_body_b.is_static {
                continue;
            }
        }
    }
}

// use gust_components::Component::{ColliderComponent, MaterialComponent, RigidBodyComponent, TransformComponent, VelocityComponent};
// use gust_components::components::transform_component::TransformComponentImpl;
// use gust_components::components::velocity_component::VelocityComponentImpl;
// use gust_components::ComponentType::{ColliderComponentType, MaterialComponentType, RigidBodyComponentType, TransformComponentType, VelocityComponentType};
// use gust_components::physics::collider_component::{ColliderComponentImpl, ColliderType};
// use gust_components::physics::material_component::MaterialComponentImpl;
// use gust_components::physics::rigid_body_component::RigidBodyComponentImpl;
// use gust_hierarchy::world::World;
// use gust_math::vectors::vect3::Vect3;
//
// use crate::physics_systems::collision_detection_system::CollisionPair;
//
// pub struct CollisionResolutionSystem;
//
// impl CollisionResolutionSystem {
//     pub fn run(delta_time: f32, world: &mut World, collision_pairs: Vec<CollisionPair>) {
//         let component_types = vec![
//             TransformComponentType,
//             RigidBodyComponentType,
//             VelocityComponentType,
//             MaterialComponentType,
//             ColliderComponentType,
//         ];
//
//         for collision in collision_pairs {
//             let (entity_a, entity_b) = collision.get_entities();
//
//             // Get components for entity A
//             let components_a = world.get_components_mut(entity_a, component_types.clone());
//             if components_a.is_none() {
//                 continue;
//             }
//             let [TransformComponent(ref mut transform_a),
//             RigidBodyComponent(ref mut rigid_body_a),
//             VelocityComponent(ref mut velocity_a),
//             MaterialComponent(ref material_a),
//             ColliderComponent(ref collider_a)] = components_a.unwrap()[..] else { continue };
//
//             // Calculate collision normal and penetration depth first
//             let (normal, penetration) = {
//                 // Temporarily borrow only `entity_b` to get collider for calculating collision info
//                 let components_b = world.get_components_mut(entity_b, vec![ColliderComponentType, TransformComponentType]);
//                 if components_b.is_none() {
//                     continue;
//                 }
//
//                 let [TransformComponent(ref transform_b),
//                 ColliderComponent(ref collider_b)] = components_b.unwrap()[..] else { continue };
//
//                 Self::calculate_collision_info(transform_a.clone(), collider_a.clone(), transform_b.clone(), collider_b.clone())
//             };
//
//             // Now get the components for entity B after the calculation
//             let components_b = world.get_components_mut(entity_b, component_types.clone());
//             if components_b.is_none() {
//                 continue;
//             }
//             let [TransformComponent(ref mut transform_b),
//             RigidBodyComponent(ref mut rigid_body_b),
//             VelocityComponent(ref mut velocity_b),
//             MaterialComponent(ref material_b),
//             ColliderComponent(ref collider_b)] = components_b.unwrap()[..] else { continue };
//
//             // Skip if both are static
//             if rigid_body_a.clone().is_static && rigid_body_b.clone().is_static {
//                 continue;
//             }
//
//             // Resolve collision
//             Self::resolve_collision(
//                 rigid_body_a, velocity_a, transform_a, material_a,
//                 rigid_body_b, velocity_b, transform_b, material_b,
//                 normal, penetration,
//             );
//         }
//     }
//
//
//     fn calculate_collision_info(
//         transform_a: TransformComponentImpl,
//         collider_a: ColliderComponentImpl,
//         transform_b: TransformComponentImpl,
//         collider_b: ColliderComponentImpl,
//     ) -> (Vect3, f32) {
//         // For AABB vs. AABB, calculate the collision normal and penetration depth
//         // This is a simplified example; for robust collision handling, more precise calculations are needed
//
//         // Transform collider bounds to world space
//         let a_min = transform_a.position + match &collider_a.collider_type {
//             ColliderType::AABB { min, .. } => *min,
//             _ => Vect3::zeros(),
//         };
//         let a_max = transform_a.position + match &collider_a.collider_type {
//             ColliderType::AABB { max, .. } => *max,
//             _ => Vect3::zeros(),
//         };
//
//         let b_min = transform_b.position + match &collider_b.collider_type {
//             ColliderType::AABB { min, .. } => *min,
//             _ => Vect3::zeros(),
//         };
//         let b_max = transform_b.position + match &collider_b.collider_type {
//             ColliderType::AABB { max, .. } => *max,
//             _ => Vect3::zeros(),
//         };
//
//         // Calculate overlap on each axis
//         let overlap_x = (a_max.x - b_min.x).min(b_max.x - a_min.x);
//         let overlap_y = (a_max.y - b_min.y).min(b_max.y - a_min.y);
//         let overlap_z = (a_max.z - b_min.z).min(b_max.z - a_min.z);
//
//         // Find the axis of least penetration
//         let penetration = overlap_x.min(overlap_y.min(overlap_z));
//
//         let normal = if penetration == overlap_x {
//             if a_max.x > b_min.x {
//                 Vect3::new(1.0, 0.0, 0.0)
//             } else {
//                 Vect3::new(-1.0, 0.0, 0.0)
//             }
//         } else if penetration == overlap_y {
//             if a_max.y > b_min.y {
//                 Vect3::new(0.0, 1.0, 0.0)
//             } else {
//                 Vect3::new(0.0, -1.0, 0.0)
//             }
//         } else {
//             if a_max.z > b_min.z {
//                 Vect3::new(0.0, 0.0, 1.0)
//             } else {
//                 Vect3::new(0.0, 0.0, -1.0)
//             }
//         };
//
//         (normal, penetration)
//     }
//
//     fn resolve_collision(
//         rigid_body_a: &RigidBodyComponentImpl,
//         velocity_a: &mut VelocityComponentImpl,
//         transform_a: &mut TransformComponentImpl,
//         material_a: &MaterialComponentImpl,
//         rigid_body_b: &RigidBodyComponentImpl,
//         velocity_b: &mut VelocityComponentImpl,
//         transform_b: &mut TransformComponentImpl,
//         material_b: &MaterialComponentImpl,
//         normal: Vect3,
//         penetration: f32,
//     ) {
//         // Calculate relative velocity
//         let relative_velocity = velocity_b.velocity - velocity_a.velocity;
//         let velocity_along_normal = relative_velocity.dot(&normal);
//
//         // Do not resolve if velocities are separating
//         if velocity_along_normal > 0.0 {
//             return;
//         }
//
//         // Calculate restitution
//         let restitution = material_a.restitution.min(material_b.restitution);
//
//         // Calculate impulse scalar
//         let inv_mass_a = if rigid_body_a.is_static { 0.0 } else { 1.0 / rigid_body_a.mass };
//         let inv_mass_b = if rigid_body_b.is_static { 0.0 } else { 1.0 / rigid_body_b.mass };
//
//         let impulse_magnitude = -(1.0 + restitution) * velocity_along_normal / (inv_mass_a + inv_mass_b);
//
//         let impulse = impulse_magnitude * normal;
//
//         // Apply impulse
//         if !rigid_body_a.is_static {
//             velocity_a.velocity -= impulse * inv_mass_a;
//         }
//         if !rigid_body_b.is_static {
//             velocity_b.velocity += impulse * inv_mass_b;
//         }
//
//         // Positional correction to prevent sinking due to gravity
//         let percent = 0.8; // Penetration percentage to correct
//         let correction = (penetration / (inv_mass_a + inv_mass_b)) * percent * normal;
//
//         if !rigid_body_a.is_static {
//             transform_a.position -= correction * inv_mass_a;
//         }
//         if !rigid_body_b.is_static {
//             transform_b.position += correction * inv_mass_b;
//         }
//
//         // Apply friction (simplified)
//         let friction = material_a.friction.min(material_b.friction);
//
//         let tangent = (relative_velocity - (velocity_along_normal * normal)).normalize();
//
//         let jt = -relative_velocity.dot(&tangent);
//         let friction_impulse_magnitude = jt / (inv_mass_a + inv_mass_b);
//
//         let friction_impulse = friction_impulse_magnitude * tangent * friction;
//
//         if !rigid_body_a.is_static {
//             velocity_a.velocity -= friction_impulse * inv_mass_a;
//         }
//         if !rigid_body_b.is_static {
//             velocity_b.velocity += friction_impulse * inv_mass_b;
//         }
//     }
// }