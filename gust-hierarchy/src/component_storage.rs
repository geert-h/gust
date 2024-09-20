use std::any::Any;
use std::collections::HashMap;

use gust_core::entity::Entity;

pub struct ComponentStorage {
    pub components: HashMap<Entity, Vec<Box<dyn Any>>>,
}

impl ComponentStorage {
    pub fn new() -> Self {
        ComponentStorage {
            components: HashMap::new(),
        }
    }

    pub fn add_component<T: 'static>(&mut self, entity: Entity, component: T) {
        self.components
            .entry(entity)
            .or_insert_with(Vec::new)
            .push(Box::new(component));
    }

    pub fn get_component<T: 'static>(&self, entity: Entity) -> Option<&T> {
        self.components.get(&entity)?.iter().find_map(|component| component.downcast_ref())
    }

    pub fn get_component_mut<T: 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        self.components.get_mut(&entity)?.iter_mut().find_map(|component| component.downcast_mut())
    }

    pub fn has_component<T: 'static>(&self, entity: Entity) -> bool {
        self.get_component::<T>(entity).is_some()
    }

    pub fn component_count(&self) -> usize {
        self.components.len()
    }
}

#[cfg(test)]
mod tests {
    use gust_core::entity::Entity;

    use crate::component_storage::ComponentStorage;

    #[derive(Debug, Clone, PartialEq)]
    struct Transform {
        position: (f32, f32),
    }

    #[derive(Debug, Clone, PartialEq)]
    struct Velocity {
        speed: f32,
    }

    #[test]
    fn test_add_component() {
        let mut storage = ComponentStorage::new();
        let entity = Entity(0);

        let transform = Transform { position: (0.0, 0.0) };
        storage.add_component(entity, transform);

        let velocity = Velocity { speed: 1.0 };
        storage.add_component(entity, velocity);

        assert_eq!(storage.components.len(), 1);
    }

    #[test]
    fn test_get_component() {
        let mut storage = ComponentStorage::new();
        let entity = Entity(0);

        let transform = Transform { position: (0.0, 0.0) };
        storage.add_component(entity, transform.clone());

        let velocity = Velocity { speed: 1.0 };
        storage.add_component(entity, velocity.clone());

        let transform_component = storage.get_component::<Transform>(entity);
        assert_eq!(transform_component, Some(&transform));

        let velocity_component = storage.get_component::<Velocity>(entity);
        assert_eq!(velocity_component, Some(&velocity));
    }

    #[test]
    fn test_get_component_mut() {
        let mut storage = ComponentStorage::new();
        let entity = Entity(0);

        let mut transform = Transform { position: (0.0, 0.0) };
        storage.add_component(entity, transform.clone());

        let mut velocity = Velocity { speed: 1.0 };
        storage.add_component(entity, velocity.clone());

        let mut transform_component = storage.get_component_mut::<Transform>(entity);

        if let Some(ref mut transform_component) = transform_component {
            transform_component.position = (1.0, 1.0);
        }

        transform.position = (1.0, 1.0);

        assert_eq!(transform_component, Some(&mut transform));

        let velocity_component = storage.get_component_mut::<Velocity>(entity);
        assert_eq!(velocity_component, Some(&mut velocity));
    }

    #[test]
    fn test_has_component() {
        let mut storage = ComponentStorage::new();
        let entity = Entity(0);

        let transform = Transform { position: (0.0, 0.0) };
        storage.add_component(entity, transform);

        let velocity = Velocity { speed: 1.0 };
        storage.add_component(entity, velocity);

        assert!(storage.has_component::<Transform>(entity));
        assert!(storage.has_component::<Velocity>(entity));
    }
}