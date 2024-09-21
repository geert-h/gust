use std::collections::HashMap;

use gust_core::entity::Entity;

use crate::{Component, ComponentType};

pub struct ComponentStorage {
    pub entity_components: HashMap<Entity, Vec<Component>>,
}

impl ComponentStorage {
    pub fn new() -> Self {
        ComponentStorage {
            entity_components: HashMap::new(),
        }
    }

    pub fn add_component(&mut self, entity: Entity, component: Component) {
        self.entity_components
            .entry(entity)
            .or_insert_with(Vec::new)
            .push(component);
    }

    pub fn get_component(&self, entity: Entity, component_type: ComponentType) -> Option<&Component> {
        self.entity_components
            .get(&entity)
            .and_then(|components| {
                components.iter().find_map(|component| {
                    if component.get_type() == component_type {
                        Some(component)
                    } else {
                        None
                    }
                })
            })
    }

    pub fn get_component_mut(&mut self, entity: Entity, component_type: ComponentType) -> Option<&mut Component> {
        self.entity_components
            .get_mut(&entity)
            .and_then(|components| {
                components.iter_mut().find_map(|component| {
                    if component.get_type() == component_type {
                        Some(component)
                    } else {
                        None
                    }
                })
            })
    }

    pub fn get_components(&self, entity: Entity) -> Option<&Vec<Component>> {
        self.entity_components.get(&entity)
    }

    pub fn has_components(&self, entity: Entity) -> bool {
        self.entity_components.contains_key(&entity)
    }

    pub fn has_component(&self, entity: Entity, component_type: ComponentType) -> bool {
        self.entity_components
            .get(&entity)
            .map(|components| {
                components.iter().any(|component| component.get_type() == component_type)
            })
            .unwrap_or(false)
    }

    pub fn query_one(&self, component_type: ComponentType) -> impl Iterator<Item=(Entity, &Component)> {
        self.entity_components
            .iter()
            .filter_map(move |(entity, components)| {
                components.iter().find(|component| component.get_type() == component_type)
                    .map(|component| (*entity, component))
            })
    }

    pub fn query_one_mut(&mut self, component_type: ComponentType) -> impl Iterator<Item=(Entity, &mut Component)> {
        self.entity_components
            .iter_mut()
            .filter_map(move |(entity, components)| {
                components.iter_mut().find(|component| component.get_type() == component_type)
                    .map(|component| (*entity, component))
            })
    }

    pub fn query(&self, component_types: Vec<ComponentType>) -> impl Iterator<Item=(Entity, Vec<&Component>)> {
        self.entity_components
            .iter()
            .filter_map(move |(entity, components)| {
                let matching_components: Vec<&Component> = components.iter()
                    .filter(|component| component_types.contains(&component.get_type()))
                    .collect();
                if matching_components.len() == component_types.len() {
                    Some((*entity, matching_components))
                } else {
                    None
                }
            })
    }

    pub fn query_mut(&mut self, component_types: Vec<ComponentType>) -> impl Iterator<Item=(Entity, Vec<&mut Component>)> {
        self.entity_components
            .iter_mut()
            .filter_map(move |(entity, components)| {
                let matching_components: Vec<&mut Component> = components.iter_mut()
                    .filter(|component| component_types.contains(&component.get_type()))
                    .collect();
                if matching_components.len() == component_types.len() {
                    Some((*entity, matching_components))
                } else {
                    None
                }
            })
    }
}