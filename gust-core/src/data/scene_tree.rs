use crate::data::scene_object::SceneObject;
use crate::data::tree::Tree;

pub struct SceneTree {
    root: Tree<SceneObject>,
}