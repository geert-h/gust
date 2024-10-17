use crate::Component::*;
use crate::components::camera_component::CameraComponentImpl;
use crate::components::mesh_component::MeshComponentImpl;
use crate::components::player_component::PlayerComponentImpl;
use crate::components::texture_component::TextureComponentImpl;
use crate::components::transform_component::TransformComponentImpl;
use crate::components::velocity_component::VelocityComponentImpl;
use crate::ComponentType::*;
use crate::physics::collider_component::ColliderComponentImpl;
use crate::physics::material_component::MaterialComponentImpl;
use crate::physics::rigid_body_component::RigidBodyComponentImpl;

pub mod component_storage;
pub mod components;
pub mod physics;

#[derive(Debug, Clone)]
pub enum Component {
    TransformComponent(TransformComponentImpl),
    VelocityComponent(VelocityComponentImpl),
    PlayerComponent(PlayerComponentImpl),
    MeshComponent(MeshComponentImpl),
    TextureComponent(TextureComponentImpl),
    CameraComponent(CameraComponentImpl),
    ColliderComponent(ColliderComponentImpl),
    MaterialComponent(MaterialComponentImpl),
    RigidBodyComponent(RigidBodyComponentImpl),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComponentType {
    TransformComponentType,
    VelocityComponentType,
    PlayerComponentType,
    MeshComponentType,
    TextureComponentType,
    CameraComponentType,
    ColliderComponentType,
    MaterialComponentType,
    RigidBodyComponentType,
}

impl Component {
    pub fn get_type(&self) -> ComponentType {
        match self {
            TransformComponent(..) => TransformComponentType,
            VelocityComponent(..) => VelocityComponentType,
            PlayerComponent(..) => PlayerComponentType,
            MeshComponent(..) => MeshComponentType,
            TextureComponent(..) => TextureComponentType,
            CameraComponent(..) => CameraComponentType,
            ColliderComponent(..) => ColliderComponentType,
            MaterialComponent(..) => MaterialComponentType,
            RigidBodyComponent(..) => RigidBodyComponentType,
        }
    }
}

impl PartialEq for Component {
    fn eq(&self, other: &Self) -> bool {
        self.get_type() == other.get_type()
    }
}
