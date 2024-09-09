use std::collections::HashSet;

use gust_ecs::component_storage::ComponentStorage;
use gust_ecs::entity::Entity;

use crate::scene_tree::SceneTree;

pub struct World {
    pub component_storage: ComponentStorage,
    pub scene_tree: SceneTree,
    pub entities: HashSet<Entity>,
    pub entity_count: usize,
}

impl World {
    pub fn new() -> Self {
        World {
            component_storage: ComponentStorage::new(),
            scene_tree: SceneTree::new(),
            entities: HashSet::new(),
            entity_count: 0,
        }
    }

    // Create a new entity
    pub fn create_entity(&mut self) -> Entity {
        let entity = Entity(self.entity_count as u32);
        self.entity_count += 1;
        self.entities.insert(entity);
        entity
    }

    // // Add a component to an entity
    // pub fn add_component(&mut self, entity: Entity, component: Component) {
    //     self.component_storage.add_component(entity, component);
    // }
    //
    // // Get a component by its type
    // pub fn get_component(&self, entity: Entity, component_type: ComponentType) -> Option<&Component> {
    //     self.component_storage.get_component(entity, component_type)
    // }
    //
    // // Get a mutable component by its type
    // pub fn get_component_mut(&mut self, entity: Entity, component_type: ComponentType) -> Option<&mut Component> {
    //     self.component_storage.get_component_mut(entity, component_type)
    // }
    //
    // // Check if an entity has a component
    // pub fn has_component(&self, entity: Entity, component_type: ComponentType) -> bool {
    //     self.component_storage.has_component(entity, component_type)
    // }
    //
    // // Query entities that have all the specified component types
    // pub fn query(&self, component_types: Vec<ComponentType>) -> Vec<Entity> {
    //     self.component_storage.query(component_types)
    // }
}
