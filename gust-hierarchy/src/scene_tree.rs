use std::collections::HashMap;

use gust_ecs::entity::Entity;

use crate::node::Node;

#[derive(Debug)]
pub struct SceneTree {
    nodes: HashMap<Entity, Node>, // Stores all nodes (entities)
}

impl SceneTree {
    pub fn new() -> Self {
        SceneTree {
            nodes: HashMap::new(),
        }
    }

    // Add a new entity to the scene tree
    pub fn add_entity(&mut self, entity: Entity) {
        let node = Node::new(entity);
        self.nodes.insert(entity, node);
    }

    // Set a parent-child relationship between two entities
    pub fn set_parent(&mut self, parent: Entity, child: Entity) {
        if let Some(child_node) = self.nodes.get_mut(&child) {
            child_node.parent = Some(parent);
        }
        if let Some(parent_node) = self.nodes.get_mut(&parent) {
            parent_node.children.push(child);
        }
    }

    // Get the parent of an entity
    pub fn get_parent(&self, entity: Entity) -> Option<Entity> {
        self.nodes.get(&entity)?.parent
    }

    // Get the children of an entity
    pub fn get_children(&self, entity: Entity) -> Option<&Vec<Entity>> {
        self.nodes.get(&entity).map(|node| &node.children)
    }
}