use gust_math::vectors::vect3::Vect3;

#[derive(Copy, Clone, Debug)]
pub struct RigidBodyComponentImpl {
    pub mass: f32,
    pub forces: Vect3,
    pub acceleration: Vect3,
    pub is_static: bool,
}