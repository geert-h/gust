use std::fmt::Debug;

use gust_math::matrices::mat4::Mat4;
use gust_math::vectors::vect3::Vect3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Component {
    TransformComponent(Transform),
    VelocityComponent(Velocity),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ComponentType {
    TransformType,
    VelocityType,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transform(pub Mat4);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Velocity(pub Vect3);
