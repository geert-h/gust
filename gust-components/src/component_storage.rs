use std::collections::HashMap;

use gust_core::entity::Entity;

use crate::Component;

pub struct ComponentStorage {
    pub storage: HashMap<Entity, Vec<Component>>,
}