use gust_math::tree::Tree;

use crate::scene::scene_object::SceneObject;

pub struct SceneTree {
    root: Tree<SceneObject>,
}