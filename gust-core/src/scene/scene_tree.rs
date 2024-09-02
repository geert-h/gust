use std::cell::RefCell;
use std::rc::{Rc, Weak};
use glium::Display;
use glium::glutin::surface::WindowSurface;
use gust_math::matrices::mat4::Mat4;
use crate::primitives::mesh::Mesh;
use crate::systems::game::Game;

pub struct SceneTree<T: GameTreeObject + ?Sized> {
    pub root: Rc<RefCell<Node<T>>>,
    pub viewer: Rc<RefCell<Node<T>>>,
}

impl<T: GameTreeObject> SceneTree<T> {
    pub fn new() -> Self {
        // Create a new root node
        let root = Node::new(T::new());
        let viewer = Node::new(T::new());

        SceneTree {
            root: root.clone(),
            viewer: viewer.clone(),
        }
    }

    pub fn add_child(&mut self, parent: Rc<RefCell<Node<T>>>, child: Rc<RefCell<Node<T>>>) {
        parent.borrow_mut().add_child(child);
        child.borrow_mut().set_parent(parent);
    }

    pub fn set_viewer(&mut self, viewer: Rc<RefCell<Node<T>>>) {
        self.viewer = viewer;
    }

    pub fn get_viewer(&self) -> Rc<RefCell<Node<T>>> {
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

pub struct Node<T: ?Sized> {
    object: Box<T>,
    parent: Weak<RefCell<Node<T>>>,
    children: Vec<Rc<RefCell<Node<T>>>>,
}

impl<T: GameTreeObject> Node<T> {
    pub fn new(object: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            object,
            parent: Weak::new(),
            children: Vec::new(),
        }))
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Node<T>>>) {
        self.children.push(child);
    }

    pub fn remove_child(&mut self, child: Rc<RefCell<Node<T>>>) {
        self.children.retain(|c| Rc::ptr_eq(c, &child));
    }

    pub fn get_children(&self) -> Vec<Rc<RefCell<Node<T>>>> {
        self.children.clone()
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<Node<T>>>> {
        self.parent.upgrade()
    }

    pub fn set_parent(&mut self, parent: Rc<RefCell<Node<T>>>) {
        self.parent = Rc::downgrade(&parent);
    }
}