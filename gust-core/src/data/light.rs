use gust_math::vectors::vect::Vect;

use crate::data::color::Color;

pub struct Light {
    pub id: u32,
    pub position: Vect,
    pub color: Color,
    pub intensity: f32,
}