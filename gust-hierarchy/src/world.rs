use std::any::TypeId;
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

    pub fn query<T: 'static>(&self) -> impl Iterator<Item=(Entity, &T)> + '_ {
        self.entities
            .iter()
            .filter_map(move |&entity| {
                self.get_component::<T>(entity)
                    .map(|component| (entity, component))
            })
    }

    pub fn query2<T: 'static, U: 'static>(&self) -> impl Iterator<Item=(Entity, (&T, &U))> + '_ {
        self.entities
            .iter()
            .filter_map(move |&entity| {
                let component1 = self.get_component::<T>(entity)?;
                let component2 = self.get_component::<U>(entity)?;
                Some((entity, (component1, component2)))
            })
    }

    pub fn query_mut<T: 'static>(&mut self) -> impl Iterator<Item=(Entity, &mut T)> + '_ {
        let mut components = &mut self.component_storage.components;

        // Use `iter_mut` to get mutable references to the component vectors
        components.iter_mut().filter_map(|(&entity, components_vec)| {
            // Find a mutable reference to the component of type T
            components_vec
                .iter_mut()
                .find_map(|component_box| component_box.downcast_mut::<T>())
                .map(|component| (entity, component))
        })
    }

    pub fn query_mut2<A: 'static, B: 'static>(
        &mut self,
    ) -> impl Iterator<Item=(Entity, (&mut A, &mut B))> + '_ {
        let components = &mut self.component_storage.components;

        components.iter_mut().filter_map(|(&entity, components_vec)| {
            // If A and B are the same type, we cannot have two mutable references to the same component
            if TypeId::of::<A>() == TypeId::of::<B>() {
                return None;
            }

            // Initialize mutable references to None
            let mut index_a: Option<usize> = None;
            let mut index_b: Option<usize> = None;

            // Collect indices of components in one iteration
            for (index, component_box) in components_vec.iter_mut().enumerate() {
                let type_id = component_box.type_id();

                if index_a.is_none() && type_id == TypeId::of::<A>() {
                    index_a = Some(index);
                }

                if index_b.is_none() && type_id == TypeId::of::<B>() {
                    index_b = Some(index);
                }

                if index_a.is_some() && index_b.is_some() {
                    break;
                }
            }

            // If both components are found
            if let (Some(index_a), Some(index_b)) = (index_a, index_b) {
                // Ensure indices are different to avoid aliasing mutable references
                if index_a != index_b {
                    // Use safe function to get mutable references to different indices
                    if let Some((comp_a_box, comp_b_box)) = Self::get_two_mut(components_vec, index_a, index_b) {
                        let component_a = comp_a_box.downcast_mut::<A>().unwrap();
                        let component_b = comp_b_box.downcast_mut::<B>().unwrap();
                        return Some((entity, (component_a, component_b)));
                    }
                }
            }

            None
        })
    }

    fn get_two_mut<T>(
        slice: &mut [T],
        idx1: usize,
        idx2: usize,
    ) -> Option<(&mut T, &mut T)> {
        if idx1 == idx2 {
            None
        } else if idx1 < idx2 {
            let (left, right) = slice.split_at_mut(idx2);
            Some((&mut left[idx1], &mut right[0]))
        } else {
            let (left, right) = slice.split_at_mut(idx1);
            Some((&mut right[0], &mut left[idx2]))
        }
    }

    pub fn query_one_entity<T: 'static>(&self) -> Option<Entity> {
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

        let entity = world.query_one_entity::<Velocity>();
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