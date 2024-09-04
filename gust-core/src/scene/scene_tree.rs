use std::cell::RefCell;
use std::rc::{Rc, Weak};
use glium::Display;
use glium::glutin::surface::WindowSurface;
use gust_math::matrices::mat4::Mat4;
use crate::components::empty_object::EmptyObject;
use crate::primitives::mesh::Mesh;
use crate::systems::game::Game;

pub struct SceneTree {
    pub root: Rc<RefCell<Node>>,
    pub viewer: Rc<RefCell<Node>>,
}

impl SceneTree {
    pub fn new() -> Self {
        // Create a new root node
        let root = Node::new(Box::new(EmptyObject {}));
        let viewer = Node::new(Box::new(EmptyObject {}));

        SceneTree {
            root: root.clone(),
            viewer: viewer.clone(),
        }
    }

    pub fn add_child(&mut self, parent: Rc<RefCell<Node>>, child: Rc<RefCell<Node>>) {
        parent.borrow_mut().add_child(&child);
        child.borrow_mut().set_parent(parent);
    }

    pub fn set_viewer(&mut self, viewer: Rc<RefCell<Node>>) {
        self.viewer = viewer;
    }

    pub fn get_viewer(&self) -> Rc<RefCell<Node>> {
        self.viewer.clone()
    }
}

pub trait GameTreeObject {
    fn update(&mut self, game: &mut Game);

    fn render(&self);

    fn get_model_matrix(&self) -> Mat4;

    fn get_mesh(&self) -> Rc<Mesh>;

    fn get_texture(&self, display: Display<WindowSurface>) -> glium::texture::Texture2d;
}

pub struct Node {
    object: Box<dyn GameTreeObject>,
    parent: Weak<RefCell<Node>>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(object: Box<dyn GameTreeObject>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            object,
            parent: Weak::new(),
            children: Vec::new(),
        }))
    }

    pub fn add_child(&mut self, child: &Rc<RefCell<Node>>) {
        self.children.push(Rc::clone(child));
    }

    pub fn remove_child(&mut self, child: Rc<RefCell<Node>>) {
        self.children.retain(|c| Rc::ptr_eq(c, &child));
    }

    pub fn get_children(&self) -> Vec<Rc<RefCell<Node>>> {
        self.children.clone()
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<Node>>> {
        self.parent.upgrade()
    }

    pub fn set_parent(&mut self, parent: Rc<RefCell<Node>>) {
        self.parent = Rc::downgrade(&parent);
    }
}