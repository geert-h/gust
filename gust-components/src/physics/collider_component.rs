use gust_math::matrices::mat3::Mat3;
use gust_math::vectors::vect3::Vect3;

#[derive(Copy, Clone, Debug)]
pub struct ColliderComponentImpl {
    collider_type: ColliderType,
}

#[derive(Copy, Clone, Debug)]
pub enum ColliderType {
    Sphere { radius: f32 },
    AABB { min: Vect3, max: Vect3 },
    OBB { center: Vect3, axes: Mat3, half_extents: Vect3 },
}