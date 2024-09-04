use gust_core::systems::game::Game;
use crate::node_trait::NodeTrait;

pub struct World {
    pub nodes: Vec<dyn NodeTrait>,
}

impl World {
    pub fn new() -> Self {
        World {
            nodes: Vec::new()
        }
    }

    pub fn add_node(&mut self, node: Box<dyn NodeTrait>) {
        self.nodes.push(*node);
    }

    pub fn update(&mut self, game: &mut Game) {
        for node in self.nodes.iter_mut() {
            node.update(game);
        }
    }

    pub fn render(&self) {
        for node in self.nodes.iter() {
            node.render();
        }
    }
}