use gust_math::matrices::matrix::Matrix;
use gust_math::vectors::vect::Vect;

use crate::data::color::Color;
use crate::data::mesh::Mesh;

pub struct GameObject {
    pub id: u32,
    pub name: String,
    pub position: Vect,
    pub rotation: Matrix,
    pub scale: f32,
    pub mesh: Mesh,
    pub color: Color,
}
