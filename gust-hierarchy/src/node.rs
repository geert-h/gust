use gust_core::entity::Entity;

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
