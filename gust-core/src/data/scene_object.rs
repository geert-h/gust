use crate::data::game_object::GameObject;
use crate::data::light::Light;

pub enum SceneObject {
    Light(Light),
    GameObject(GameObject),
}