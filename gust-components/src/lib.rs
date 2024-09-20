use crate::components::camera_component::CameraComponent;
use crate::components::mesh_component::MeshComponent;
use crate::components::player_component::PlayerComponent;
use crate::components::texture_component::TextureComponent;
use crate::components::transform_component::TransformComponent;
use crate::components::velocity_component::VelocityComponent;

mod component_storage;
pub mod components;

pub enum Component {
    TransformComponent(TransformComponent),
    VelocityComponent(VelocityComponent),
    PlayerComponent(PlayerComponent),
    MeshComponent(MeshComponent),
    TextureComponent(TextureComponent),
    CameraComponent(CameraComponent),
}

pub enum ComponentType {
    TransformComponent,
    VelocityComponent,
    PlayerComponent,
    MeshComponent,
    TextureComponent,
    CameraComponent,
}

impl Component {
    pub fn get_type(&self) -> ComponentType {
        match self {
            Component::TransformComponent(..) => ComponentType::TransformComponent,
            Component::VelocityComponent(..) => ComponentType::VelocityComponent,
            Component::PlayerComponent(..) => ComponentType::PlayerComponent,
            Component::MeshComponent(..) => ComponentType::MeshComponent,
            Component::TextureComponent(..) => ComponentType::TextureComponent,
            Component::CameraComponent(..) => ComponentType::CameraComponent,
        }
    }
}