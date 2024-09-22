use crate::components::camera_component::CameraComponentImpl;
use crate::components::mesh_component::MeshComponentImpl;
use crate::components::player_component::PlayerComponentImpl;
use crate::components::texture_component::TextureComponentImpl;
use crate::components::transform_component::TransformComponentImpl;
use crate::components::velocity_component::VelocityComponentImpl;
use crate::ComponentType::{CameraComponentType, MeshComponentType, PlayerComponentType, TextureComponentType, TransformComponentType, VelocityComponentType};

pub mod component_storage;
pub mod components;

#[derive(Debug, Clone)]
pub enum Component {
    TransformComponent(TransformComponentImpl),
    VelocityComponent(VelocityComponentImpl),
    PlayerComponent(PlayerComponentImpl),
    MeshComponent(MeshComponentImpl),
    TextureComponent(TextureComponentImpl),
    CameraComponent(CameraComponentImpl),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComponentType {
    TransformComponentType,
    VelocityComponentType,
    PlayerComponentType,
    MeshComponentType,
    TextureComponentType,
    CameraComponentType,
}

impl Component {
    pub fn get_type(&self) -> ComponentType {
        match self {
            Component::TransformComponent(..) => TransformComponentType,
            Component::VelocityComponent(..) => VelocityComponentType,
            Component::PlayerComponent(..) => PlayerComponentType,
            Component::MeshComponent(..) => MeshComponentType,
            Component::TextureComponent(..) => TextureComponentType,
            Component::CameraComponent(..) => CameraComponentType,
        }
    }
}