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
        if world.scene_tree.get_parent(*entity).is_none() {
            queue.push_back(*entity);
        }
    }

    // Breadth-first traversal to propagate transforms
    while let Some(entity) = queue.pop_front() {
        // Get the parent's transform if it exists
        if let Some(parent) = world.scene_tree.get_parent(entity) {
            if let (Some(TransformComponent(parent_transform)), Some(TransformComponent(entity_transform))) = (
                world.get_component(parent, TransformType),
                world.get_component_mut(entity, TransformType),
            ) {
                // Calculate global transform by combining parent's global transform and entity's local transform
                *entity_transform = combine_transforms(parent_transform, entity_transform);
            }
        }

        // Enqueue children for processing
        if let Some(children) = world.scene_tree.get_children(entity) {
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