use std::collections::HashSet;

use crate::component_storage::ComponentStorage;
use crate::entity::Entity;
use crate::scene_tree::SceneTree;

/// The World struct is the main struct that holds all the entities and components.
/// It is responsible for creating, adding, and removing entities and components.
/// It also provides methods to query entities based on their components.
///
/// # Example
///
/// ```
/// struct Transform {
///    position: (f32, f32),
/// }
///
/// struct Velocity {
///   speed: f32,
/// }
///
/// use gust_hierarchy::world::World;
/// use gust_hierarchy::entity::Entity;
///
/// let mut world = World::new();
/// let entity = world.create_entity();
///
/// let transform = Transform { position: (0.0, 0.0) };
/// world.add_component(entity, transform);
///
/// let velocity = Velocity { speed: 1.0 };
/// world.add_component(entity, velocity);
///
/// let transform_component = world.get_component::<Transform>(entity);
/// assert_eq!(transform_component, Some(&transform));
///
/// let entities = world.query::<Transform>();
/// assert_eq!(entities, vec![entity]);
/// ```
pub struct World {
    component_storage: ComponentStorage,
    scene_tree: SceneTree,
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

    pub fn spawn(&mut self) -> Entity {
        let entity = self.create_entity();
        self.scene_tree.add_entity(entity);
        entity
    }

    pub fn get_parent(&self, entity: Entity) -> Option<Entity> {
        self.scene_tree.get_parent(entity)
    }

    pub fn get_children(&self, entity: Entity) -> Option<&Vec<Entity>> {
        self.scene_tree.get_children(entity)
    }

    // Add a component to an entity
    pub fn add_component<T: 'static>(&mut self, entity: Entity, component: T) {
        self.component_storage.add_component(entity, component);
    }

    // Get a component by its type
    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<&T> {
        self.component_storage.get_component::<T>(entity)
    }

    // Get a mutable component by its type
    pub fn get_component_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        self.component_storage.get_component_mut::<T>(entity)
    }

    // Check if an entity has a component
    pub fn has_component<T: 'static>(&self, entity: Entity) -> bool {
        self.component_storage.has_component::<T>(entity)
    }

    pub fn set_parent(&mut self, parent: Entity, child: Entity) {
        self.scene_tree.set_parent(parent, child);
    }

    pub fn query<T: 'static>(&mut self) -> Vec<Entity> {
        self.entities
            .iter()
            .filter(|entity| self.has_component::<T>(**entity))
            .copied()
            .collect()
    }

    pub fn query_one<T: 'static>(&self) -> Option<Entity> {
        self.entities
            .iter()
            .find(|entity| self.has_component::<T>(**entity))
            .copied()
    }

    pub fn query_one_mut<T: 'static>(&mut self) -> Option<Entity> {
        self.entities
            .iter()
            .find(|entity| self.has_component::<T>(**entity))
            .copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct Transform {
        position: (f32, f32),
    }

    #[derive(Debug, Clone, PartialEq)]
    struct Velocity {
        speed: f32,
    }

    #[test]
    fn test_create_entity() {
        let mut world = World::new();
        let entity = world.create_entity();
        assert!(world.entities.contains(&entity));
    }

    #[test]
    fn test_spawn() {
        let mut world = World::new();
        let entity = world.spawn();
        assert!(world.entities.contains(&entity));
    }

    #[test]
    fn test_add_component() {
        let mut world = World::new();
        let entity = world.spawn();

        let transform = Transform { position: (0.0, 0.0) };
        world.add_component(entity, transform);

        let velocity = Velocity { speed: 1.0 };
        world.add_component(entity, velocity);

        assert_eq!(world.component_storage.component_count(), 1);
    }

    #[test]
    fn test_get_component() {
        let mut world = World::new();
        let entity = world.spawn();

        let transform = Transform { position: (0.0, 0.0) };
        world.add_component(entity, transform.clone());

        let velocity = Velocity { speed: 1.0 };
        world.add_component(entity, velocity.clone());

        let transform_component = world.get_component::<Transform>(entity);
        assert_eq!(transform_component, Some(&transform));
    }

    #[test]
    fn test_get_component_mut() {
        let mut world = World::new();
        let entity = world.spawn();

        let transform = Transform { position: (0.0, 0.0) };
        world.add_component(entity, transform.clone());

        let velocity = Velocity { speed: 1.0 };
        world.add_component(entity, velocity.clone());

        if let Some(transform_component) = world.get_component_mut::<Transform>(entity) {
            transform_component.position = (1.0, 1.0);
        }

        let new_transform = Transform { position: (1.0, 1.0) };
        let transform_component = world.get_component::<Transform>(entity);
        assert_eq!(transform_component, Some(&new_transform));
    }

    #[test]
    fn test_has_component() {
        let mut world = World::new();
        let entity = world.spawn();

        let transform = Transform { position: (0.0, 0.0) };
        world.add_component(entity, transform);

        assert!(world.has_component::<Transform>(entity));
    }

    #[test]
    fn test_query() {
        let mut world = World::new();
        let entity1 = world.spawn();
        let entity2 = world.spawn();

        let transform = Transform { position: (0.0, 0.0) };
        world.add_component(entity1, transform);

        let velocity = Velocity { speed: 1.0 };
        world.add_component(entity2, velocity);

        let entities = world.query::<Transform>();
        assert_eq!(entities, vec![entity1]);
    }

    #[test]
    fn test_query_one() {
        let mut world = World::new();
        let entity1 = world.spawn();
        let entity2 = world.spawn();

        let transform = Transform { position: (0.0, 0.0) };
        world.add_component(entity1, transform);

        let velocity = Velocity { speed: 1.0 };
        world.add_component(entity2, velocity);

        let entity = world.query_one::<Velocity>();
        assert_eq!(entity, Some(entity2));
    }

    #[test]
    fn test_query_with_two_components() {
        let mut world = World::new();
        let entity1 = world.spawn();
        let entity2 = world.spawn();

        let transform = Transform { position: (0.0, 0.0) };
        world.add_component(entity1, transform);

        let velocity = Velocity { speed: 1.0 };
        world.add_component(entity1, velocity);

        let entities = world.query::<(Transform, Velocity)>();
        assert_eq!(entities, vec![entity1]);
    }

    #[test]
    fn test_set_parent() {
        let mut world = World::new();
        let parent = world.spawn();
        let child = world.spawn();

        world.set_parent(parent, child);

        assert_eq!(world.get_parent(child), Some(parent));
    }
}