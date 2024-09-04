use std::cell::RefCell;
use std::rc::{Rc, Weak};
use gust_core::systems::game::Game;
use gust_math::matrices::mat4::Mat4;
use crate::node_trait::NodeTrait;

pub struct Node {
    name: String,
    parent: Weak<Rc<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
    transform: Mat4,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: name.to_string(),
            parent: Weak::new(),
            children: Vec::new(),
            transform: Mat4::identity(),
        }
    }
}

impl NodeTrait for Node {
    fn update(&mut self, game: Game) {
        for child in self.children.iter() {
            child.borrow_mut().update();
        }
    }

    fn render(&self) {
        for child in self.children.iter() {
            child.borrow().render();
        }
    }

    fn get_world_transform(&self) -> Mat4 {
        let mut transform = self.transform.clone();

        if let Some(parent) = self.parent.upgrade() {
            transform = parent.borrow().get_world_transform() * transform;
        }
        transform
    }

    fn add_child(&mut self, child: Rc<RefCell<Node>>) {
        child.borrow_mut().parent = Rc::downgrade(&Rc::new(RefCell::new(self.clone())));
        self.children.push(child);
    }

    fn remove_child(&mut self, child: Rc<RefCell<Node>>) {
        if let Some(index) = self.children.iter().position(|x| Rc::ptr_eq(x, &child)) {
            self.children.remove(index);
        }
    }
}