use std::collections::VecDeque;

use gust_core::systems::game::Game;
use gust_hierarchy::entity::Entity;
use gust_hierarchy::world::World;
use gust_math::matrices::mat4::Mat4;

fn main() {
    // test();
    Game::new().run();
}

#[derive(Clone, Debug)]
struct Transform(pub Mat4);

fn test() {
    let mut world = World::new();

    // Create entities
    let parent = world.spawn();
    let child = world.spawn();
    let grand_child = world.spawn();

    // Set up the scene tree
    world.set_parent(parent, child);
    world.set_parent(parent, grand_child);
    world.set_parent(child, grand_child);

    let transform = Transform(Mat4::identity().translate([1.0, 2.0, 3.0].into()));

    // Add components
    world.add_component(parent, transform.clone());
    world.add_component(child, transform.clone());
    world.add_component(grand_child, transform.clone());

    // Propagate transforms from parent to child
    propagate_transform(&mut world);

    // Get the child's global position
    if let Some(child_transform) = world.get_component::<Transform>(child) {
        println!("Child's global position: {:?}", child_transform.0);
    }

    if let Some(parent_transform) = world.get_component::<Transform>(parent) {
        println!("Parent's global position: {:?}", parent_transform.0);
    }

    if let Some(grand_child_transform) = world.get_component::<Transform>(grand_child) {
        println!("Grand Child's global position: {:?}", grand_child_transform.0);
    }
}

pub fn propagate_transform(world: &mut World) {
    // Iterate over all entities in the scene tree
    let mut queue: VecDeque<Entity> = VecDeque::new();

    // Start from the root entities (entities without parents)
    for entity in &world.entities {
        if world.get_parent(*entity).is_none() {
            queue.push_back(*entity);
        }
    }

    // Breadth-first traversal to propagate transforms
    while let Some(entity) = queue.pop_front() {
        // Get the parent entity
        let parent = world.get_parent(entity);

        // Extract the parent transform and the entity transform as separate variables
        let (parent_transform, entity_transform) = match parent {
            Some(parent_entity) => {
                // Get the parent's transform (immutable borrow)
                let parent_transform = world.get_component(parent_entity);
                // Get the entity's transform (we won't mutate it yet)
                let entity_transform = world.get_component(entity);
                (parent_transform, entity_transform)
            }
            None => (None, world.get_component(entity)),
        };

        if let (Some(parent_transform), Some(entity_transform)) = (parent_transform, entity_transform) {
            let new_transform = combine_transforms(parent_transform, entity_transform);

            if let Some(entity_transform_mut) = world.get_component_mut(entity) {
                *entity_transform_mut = new_transform;
            }
        }

        // Enqueue children for processing
        let children = world.get_children(entity);
        if let Some(children) = children {
            for &child in children {
                queue.push_back(child);
            }
        }
    }
}

// Function to combine parent and local transforms
fn combine_transforms(parent_transform: &Transform, local_transform: &Transform) -> Transform {
    Transform(parent_transform.0 * local_transform.0)
}
