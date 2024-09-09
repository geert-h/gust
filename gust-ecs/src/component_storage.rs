use std::collections::HashMap;

use crate::component::{Component, ComponentType};
use crate::component::Component::{TransformComponent, VelocityComponent};
use crate::component::ComponentType::{TransformType, VelocityType};
use crate::entity::Entity;

#[derive(Debug, Clone)]
pub struct ComponentStorage {
    storages: HashMap<Entity, Vec<Component>>,
}

impl ComponentStorage {
    pub fn new() -> Self {
        ComponentStorage {
            storages: HashMap::new(),
        }
    }

    pub fn add_component(&mut self, entity: Entity, component: Component) {
        self.storages
            .entry(entity)
            .or_insert_with(Vec::new);

        // Check if the entity already has the component
        if self.storages.get(&entity).unwrap().iter().any(|c| c == &component) {
            println!("Entity already has component {:?}", component);
            return;
        }

        self.storages.get_mut(&entity).unwrap().push(component);
    }

    pub fn get_component(&self, entity: Entity, component_type: ComponentType) -> Option<&Component> {
        self.storages.get(&entity)?.iter().find(|component| match component {
            TransformComponent(_) => component_type == TransformType,
            VelocityComponent(_) => component_type == VelocityType,
        })
    }

    pub fn get_component_mut(&mut self, entity: Entity, component_type: ComponentType) -> Option<&mut Component> {
        self.storages.get_mut(&entity)?.iter_mut().find(|component| match component {
            TransformComponent(_) => component_type == TransformType,
            VelocityComponent(_) => component_type == VelocityType,
        })
    }

    pub fn has_component(&self, entity: Entity, component_type: ComponentType) -> bool {
        self.get_component(entity, component_type).is_some()
    }

    // Query for entities with specific component types
    pub fn query(&self, component_types: Vec<ComponentType>) -> Vec<Entity> {
        self.storages
            .iter()
            .filter_map(|(&entity, components)| {
                let mut has_all_components = true;

                for component_type in &component_types {
                    if !components.iter().any(|component| match component {
                        TransformComponent(_) => component_type == &TransformType,
                        VelocityComponent(_) => component_type == &VelocityType,
                    }) {
                        has_all_components = false;
                        break;
                    }
                }

                if has_all_components {
                    Some(entity)
                } else {
                    None
                }
            })
            .collect()
    }
}