use gust_math::vectors::vect3::Vect3;

use crate::data::color::Color;

pub struct Light {
    pub id: u32,
    pub position: Vect3,
    pub color: Color,
}