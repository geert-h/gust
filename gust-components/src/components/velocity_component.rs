use gust_math::vectors::vect3::Vect3;

#[derive(Debug, Clone, PartialEq)]
pub struct VelocityComponentImpl {
    pub velocity: Vect3,
}

impl VelocityComponentImpl {
    pub fn new(velocity: Vect3) -> Self {
        VelocityComponentImpl { velocity }
    }

    pub fn default() -> Self {
        VelocityComponentImpl { velocity: [0.0, 0.0, 0.0].into() }
    }
}