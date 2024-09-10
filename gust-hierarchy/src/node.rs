use std::collections::VecDeque;

use gust_ecs::component::Component::TransformComponent;
use gust_ecs::component::ComponentType::TransformType;
use gust_ecs::component::Transform;
use gust_ecs::entity::Entity;

use crate::world::World;

#[derive(Debug)]
pub struct Node {
    pub entity: Entity,
    pub parent: Option<Entity>,
    pub children: Vec<Entity>,
}

impl Node {
    pub fn new(entity: Entity) -> Self {
        Node {
            entity,
            parent: None,
            children: Vec::new(),
        }
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
                let parent_transform = world.get_component(parent_entity, TransformType);
                // Get the entity's transform (we won't mutate it yet)
                let entity_transform = world.get_component(entity, TransformType);
                (parent_transform, entity_transform)
            }
            None => (None, world.get_component(entity, TransformType)),
        };

        // Now perform the mutable borrow on the entity's transform
        if let (Some(TransformComponent(parent_transform)), Some(TransformComponent(entity_transform))) = (parent_transform, entity_transform) {
            // Combine the parent's global transform with the entity's local transform
            let new_transform = combine_transforms(parent_transform, entity_transform);

            // Now apply the new transform with a mutable borrow
            if let Some(TransformComponent(entity_transform_mut)) = world.get_component_mut(entity, TransformType) {
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