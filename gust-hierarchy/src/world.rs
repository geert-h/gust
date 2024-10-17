use std::collections::HashSet;

use gust_components::{Component, ComponentType};
use gust_components::component_storage::ComponentStorage;
use gust_core::entity::Entity;

use crate::scene_tree::SceneTree;

/// The World struct is the main struct that holds all the entities and components.
/// It is responsible for creating, adding, and removing entities and components.
/// It also provides methods to query entities based on their components.
///
/// # Example
///
/// ```
/// use gust_components::Component;
/// use gust_components::Component::{TransformComponent, VelocityComponent};
/// use gust_components::components::transform_component::TransformComponentImpl;
/// use gust_components::components::velocity_component::VelocityComponentImpl;
/// use gust_components::ComponentType::TransformComponentType;
/// use gust_hierarchy::world::World;
/// use gust_core::entity::Entity;
///
/// let mut world = World::new();
/// let entity = world.create_entity();
///
/// let transform = TransformComponentImpl::default();
/// world.add_component(entity, TransformComponent(transform));
///
/// let velocity = VelocityComponentImpl::default();
/// world.add_component(entity, VelocityComponent(velocity));
///
/// let transform_component = world.get_component(entity, TransformComponentType);
/// assert_eq!(transform_component, Some(&TransformComponent(transform)));
///
/// let entities = world.query_one(TransformComponentType).collect::<Vec<(Entity, &Component)>>().iter().map(|(entity, component)| *entity).collect::<Vec<Entity>>();
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
    pub fn add_component(&mut self, entity: Entity, component: Component) {
        self.component_storage.add_component(entity, component);
    }

    // Get a component by its type
    pub fn get_component(&self, entity: Entity, component_type: ComponentType) -> Option<&Component> {
        self.component_storage.get_component(entity, component_type)
    }

    // Get a mutable component by its type
    pub fn get_component_mut(&mut self, entity: Entity, component_type: ComponentType) -> Option<&mut Component> {
        self.component_storage.get_component_mut(entity, component_type)
    }

    pub fn get_components(&self, entity: Entity, component_types: Vec<ComponentType>) -> Option<Vec<&Component>> {
        self.component_storage.get_components(entity, component_types)
    }

    pub fn get_components_mut(&mut self, entity: Entity, component_types: Vec<ComponentType>) -> Option<Vec<&mut Component>> {
        self.component_storage.get_components_mut(entity, component_types)
    }

    // Check if an entity has a component
    pub fn has_component(&self, entity: Entity, component_type: ComponentType) -> bool {
        self.component_storage.has_component(entity, component_type)
    }

    pub fn set_parent(&mut self, parent: Entity, child: Entity) {
        self.scene_tree.set_parent(parent, child);
    }

    pub fn query_one(&self, component_type: ComponentType) -> impl Iterator<Item=(Entity, &Component)> {
        self.component_storage.query_one(component_type)
    }

    pub fn query_one_mut(&mut self, component_type: ComponentType) -> impl Iterator<Item=(Entity, &mut Component)> {
        self.component_storage.query_one_mut(component_type)
    }

    pub fn query(&self, component_types: Vec<ComponentType>) -> impl Iterator<Item=(Entity, Vec<&Component>)> {
        self.component_storage.query(component_types)
    }

    pub fn query_mut(&mut self, component_types: Vec<ComponentType>) -> impl Iterator<Item=(Entity, Vec<&mut Component>)> {
        self.component_storage.query_mut(component_types)
    }

    pub fn with_components(&self, component_types: Vec<ComponentType>) -> impl Iterator<Item=&Entity> {
        self.component_storage.with_components(component_types)
    }
}


#[cfg(test)]
mod tests {
    use gust_components::Component::{TransformComponent, VelocityComponent};
    use gust_components::components::transform_component::TransformComponentImpl;
    use gust_components::components::velocity_component::VelocityComponentImpl;
    use gust_components::ComponentType::TransformComponentType;

    use super::*;

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

        let transform = TransformComponentImpl::default();
        world.add_component(entity, TransformComponent(transform));

        let velocity = VelocityComponentImpl::default();
        world.add_component(entity, VelocityComponent(velocity));

        assert_eq!(world.entity_count, 1);
    }

    #[test]
    fn test_get_component() {
        let mut world = World::new();
        let entity = world.spawn();

        let transform = TransformComponentImpl::default();
        world.add_component(entity, TransformComponent(transform.clone()));

        let velocity = VelocityComponentImpl::default();
        world.add_component(entity, VelocityComponent(velocity));

        let transform_component = world.get_component(entity, TransformComponentType).unwrap();
        assert_eq!(transform_component, &TransformComponent(transform.clone()));
    }

    #[test]
    fn test_get_component_mut() {
        let mut world = World::new();
        let entity = world.spawn();

        let transform = TransformComponentImpl::default();
        world.add_component(entity, TransformComponent(transform.clone()));

        let velocity = VelocityComponentImpl::default();
        world.add_component(entity, VelocityComponent(velocity.clone()));

        if let Some(TransformComponent(transform_component)) = world.get_component_mut(entity, TransformComponentType) {
            transform_component.position = [0.0, 0.0, 0.0].into();
        }

        let new_transform = TransformComponent(TransformComponentImpl::default());
        let transform_component = world.get_component(entity, TransformComponentType);
        assert_eq!(transform_component, Some(&new_transform));
    }

    #[test]
    fn test_has_component() {
        let mut world = World::new();
        let entity = world.spawn();

        let transform = TransformComponentImpl::default();
        world.add_component(entity, TransformComponent(transform));

        assert!(world.has_component(entity, TransformComponentType));
    }

    #[test]
    fn test_with_components() {
        let mut world = World::new();
        let entity1 = world.spawn();
        let entity2 = world.spawn();

        let transform = TransformComponentImpl::default();
        world.add_component(entity1, TransformComponent(transform));

        let velocity = VelocityComponentImpl::default();
        world.add_component(entity2, VelocityComponent(velocity));

        let entities: Vec<_> = world.with_components(vec![TransformComponentType]).collect();
        assert_eq!(entities, vec![&entity1]);
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